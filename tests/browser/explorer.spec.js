import {test,expect} from '@playwright/test';
import {readFileSync,mkdirSync,writeFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
const references=JSON.parse(readFileSync('.cache/native-references.json','utf8'));
const matrix=JSON.parse(readFileSync('.cache/matrix-references.json','utf8'));
const workerSource=readFileSync('web/worker.js','utf8');
async function ready(page){await page.goto('./');await expect(page.locator('#counts')).toHaveText('3 selected · 1 excluded');await expect(page.locator('#status')).toContainText('Selection complete');}
async function submit(page,seed,budget){await page.locator('#seed').selectOption(seed);await page.locator('#budget').fill(budget);await page.locator('#run').click();}
async function download(page,id){const [file]=await Promise.all([page.waitForEvent('download'),page.locator(id).click()]);return readFileSync(await file.path(),'utf8');}
async function rawParity(page,rows){return page.evaluate(async rows=>{
  const worker=new Worker(new URL('./worker.js',location.href),{type:'module'});
  const wait=match=>new Promise((resolve,reject)=>{
    const timer=setTimeout(()=>{cleanup();reject(new Error('Parity worker response deadline'));},12000);
    function cleanup(){clearTimeout(timer);worker.removeEventListener('message',listener);worker.removeEventListener('error',error);}
    function error(event){cleanup();reject(new Error(event.message));}
    function listener({data}){if(data.type==='startup-error'||data.type==='compute-error'){cleanup();reject(new Error(data.message));}else if(match(data)){cleanup();resolve(data);}}
    worker.addEventListener('message',listener);worker.addEventListener('error',error);
  });
  try{await wait(data=>data.type==='ready');const mismatches=[];let compared=0;
    for(const [i,row] of rows.entries()){const pending=wait(data=>data.id===i);worker.postMessage({type:'explore',id:i,request:row.request||row.input});const data=await pending;compared++;if(data.canonical!==row.canonical)mismatches.push({index:i,request:row.request||row.input,expected:row.canonical,actual:data.canonical});if(mismatches.length>=3)break;}
    return {compared,mismatches};
  }finally{worker.terminate();}
},rows);}
test('all 1,024 admitted native reports equal executing worker WASM',async ({page})=>{
  test.setTimeout(90000);await ready(page);const result=await rawParity(page,matrix);expect(result.mismatches).toEqual([]);expect(result.compared).toBe(1024);
});
test('representative and invalid raw requests retain native canonical failures',async ({page})=>{
  await ready(page);const result=await rawParity(page,references);expect(result.mismatches).toEqual([]);expect(result.compared).toBe(references.length);
});
for(const ref of references.filter(ref=>JSON.parse(ref.canonical).ok))test(`own controls and canonical downloads: ${ref.name}`,async ({page})=>{
  const errors=[];page.on('pageerror',error=>errors.push(error.message));await ready(page);const request=JSON.parse(ref.input);await submit(page,request.seed,request.budget);const response=JSON.parse(ref.canonical);
  await expect(page.locator('#counts')).toHaveText(`${response.selected_count} selected · ${response.excluded_count} excluded`);
  expect(await download(page,'#report-download')).toBe(ref.canonical);expect(JSON.parse(await download(page,'#download'))).toEqual(response.pack);expect(JSON.parse(await download(page,'#request-download'))).toEqual(request);
  await expect(page.locator('.unit')).toHaveCount(4);await expect(page.locator('#edges li')).toHaveCount(3);
  for(const item of response.units){const card=page.locator(`.unit[data-index="${item.index}"]`);await expect(card).toHaveAttribute('data-selected',String(item.selected));await expect(card).toContainText(item.unit.source_id);await expect(card).toContainText(item.unit.text);await card.locator('summary').click();await expect(card.locator('pre')).toContainText(item.unit.content_hash);}
  if(response.over_budget)await expect(page.locator('#overrun')).toContainText(String(response.budget_overrun));else await expect(page.locator('#overrun')).toBeHidden();expect(errors).toEqual([]);
});
test('invalid budget text is refused by Rust without coercion or stale downloads',async ({page})=>{
  await ready(page);for(const value of ['','-1','+1','1.5','NaN','Infinity','0xA0','1e2',' 80 ','0','257']){
    await page.locator('#budget').fill(value);await expect(page.locator('#results')).toBeHidden();await expect(page.locator('#download')).toBeDisabled();await page.locator('#run').click();await expect(page.locator('#error')).toBeVisible();await expect(page.locator('#diagnostic')).toContainText('controls');await expect(page.locator('#budget')).toHaveValue(value);
  }
  await page.locator('#budget').fill('160');await page.locator('#run').click();await expect(page.locator('#counts')).toHaveText('3 selected · 1 excluded');
});
test('excluded notes stay inspectable and can become a new seed',async ({page})=>{
  await ready(page);await expect(page.locator('.unit[data-index="3"]')).toHaveAttribute('data-selected','false');await page.locator('.unit[data-index="3"] button').click();await expect(page.locator('#seed')).toHaveValue('3');await expect(page.locator('#results')).toBeHidden();await page.locator('#run').click();await expect(page.locator('.unit[data-index="3"]')).toHaveAttribute('data-selected','true');await expect(page.locator('.unit[data-index="3"]')).toContainText('Seed');
});
test('Restore example retains engine and changes controls only with stale export suppression',async ({page})=>{
  await ready(page);await submit(page,'2','79');await expect(page.locator('#status')).toContainText('Selection complete');await page.locator('#reset').click();await expect(page.locator('#seed')).toHaveValue('0');await expect(page.locator('#budget')).toHaveValue('160');await expect(page.locator('#seed')).toBeFocused();await expect(page.locator('#download')).toBeDisabled();await page.locator('#run').click();await expect(page.locator('#counts')).toHaveText('3 selected · 1 excluded');
});
test('public fixture, source, compiled artifact and scoped notice routes are independent',async ({page})=>{
  await ready(page);const provenance=await(await page.request.get('./provenance.json')).json();const fixture=await(await page.request.get('./fixture.json')).json();expect(fixture.units).toHaveLength(4);expect(fixture.adjacency).toEqual([[1,3],[0,2],[1],[0]]);
  for(const name of ['crates/mdcrop-core/src/lib.rs','crates/mdcrop-web/src/lib.rs','Cargo.lock']){
    const raw=await(await page.request.get('./source/'+name)).body();const expected=provenance.dirty?readFileSync(name):execFileSync('git',['show',`${provenance.commit}:${name}`]);expect(raw.equals(expected)).toBe(true);
  }
  const wasm=await page.request.get('./pkg/mdcrop_web_bg.wasm');expect(wasm.headers()['content-type']).toContain('application/wasm');expect((await wasm.body()).subarray(0,4).equals(Buffer.from([0,97,115,109]))).toBe(true);
  for(const name of ['source.zip','BUILD.md','LICENSE','THIRD-PARTY-NOTICES.txt','rust/COPYRIGHT-library.html'])expect((await page.request.get('./'+name)).ok()).toBe(true);
  const notices=await(await page.request.get('./THIRD-PARTY-NOTICES.txt')).text();for(const name of ['rstat-core','rgraph-core','ropt-core','rctx-core','slice-core','mdpath','mdport','getrandom'])expect(notices).toContain(name);
});
test('WASM fetch failure preserves own controls and independent source/fixture routes',async ({page})=>{
  await page.route('**/pkg/*.wasm',route=>route.abort());await page.goto('./');await expect(page.locator('#retry')).toBeVisible();await page.locator('#seed').selectOption('2');await page.locator('#budget').fill('120');
  for(const path of ['fixture.json','source.zip','BUILD.md'])expect((await page.request.get('./'+path)).ok()).toBe(true);
  await page.unroute('**/pkg/*.wasm');await page.locator('#retry').click();await expect(page.locator('#run')).toBeEnabled();await expect(page.locator('#seed')).toHaveValue('2');await expect(page.locator('#budget')).toHaveValue('120');await expect(page.locator('#results')).toBeHidden();await page.locator('#run').click();await expect(page.locator('#selection-summary')).toContainText('target-note');
});
test('startup deadline survives edits; late ready from terminated worker stays fenced',async ({page})=>{
  await page.addInitScript(()=>{const Actual=window.Worker;window.Worker=class extends Actual{constructor(...args){super(...args);window.testWorker=this;}};});await page.clock.install();let fetched=false;await page.route('**/pkg/*.wasm',()=>{fetched=true;});await page.goto('./');await expect.poll(()=>fetched).toBe(true);await page.locator('#budget').fill('79');await page.locator('#seed').selectOption('1');await page.clock.fastForward(13000);await expect(page.locator('#retry')).toBeVisible();await page.evaluate(()=>window.testWorker.onmessage({data:{type:'ready'}}));await expect(page.locator('#run')).toBeDisabled();await expect(page.locator('#status')).toContainText('unavailable');
  await page.unroute('**/pkg/*.wasm');await page.locator('#retry').click();await expect(page.locator('#run')).toBeEnabled();await expect(page.locator('#budget')).toHaveValue('79');await expect(page.locator('#seed')).toHaveValue('1');await page.locator('#run').click();await expect(page.locator('#status')).toContainText('Selection complete');
});
test('compute deadline survives edits and Retry preserves controls',async ({page})=>{
  await page.clock.install();await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:workerSource+'\nconst original=self.onmessage;let seen=0;self.onmessage=e=>{if(++seen===1)original(e);};'}));await ready(page);await submit(page,'2','120');await expect(page.locator('#status')).toContainText('Selecting');await page.locator('#budget').fill('79');await page.clock.fastForward(11000);await expect(page.locator('#retry')).toBeVisible();await expect(page.locator('#error-message')).toContainText('deadline');await expect(page.locator('#download')).toBeDisabled();await page.unroute('**/worker.js');await page.locator('#retry').click();await expect(page.locator('#run')).toBeEnabled();await expect(page.locator('#budget')).toHaveValue('79');await expect(page.locator('#seed')).toHaveValue('2');await page.locator('#run').click();await expect(page.locator('#status')).toContainText('Selection complete');
});
test('late real results cannot repopulate changed controls or exports',async ({page})=>{
  await page.addInitScript(()=>{const Actual=window.Worker;window.Worker=class extends Actual{constructor(...args){super(...args);window.testWorker=this;}};});await page.route('**/worker.js',route=>route.fulfill({contentType:'text/javascript',body:workerSource+'\nconst post=self.postMessage.bind(self);let count=0,pending;self.postMessage=data=>{if(data.type==="result"&&++count>1)pending=data;else post(data);};const original=self.onmessage;self.onmessage=e=>{if(e.data.type==="release"){post(pending);pending=null;}else original(e);};'}));await ready(page);await submit(page,'0','80');await expect(page.locator('#status')).toContainText('Selecting');await page.locator('#budget').fill('200');await page.evaluate(()=>window.testWorker.postMessage({type:'release'}));await expect(page.locator('#run')).toBeEnabled();await expect(page.locator('#results')).toBeHidden();await expect(page.locator('#download')).toBeDisabled();
});
test('UTF-8 request cap refuses before another worker call',async ({page})=>{
  await page.addInitScript(()=>{const Actual=window.Worker;window.Worker=class extends Actual{postMessage(...args){if(args[0].type==='explore')window.posts=(window.posts||0)+1;return super.postMessage(...args);}};});await ready(page);const before=await page.evaluate(()=>window.posts);await page.locator('#budget').fill('é'.repeat(513));await page.locator('#run').click();await expect(page.locator('#error-message')).toContainText('1,024');expect(await page.evaluate(()=>window.posts)).toBe(before);
});
for(const [name,viewport] of [['desktop',{width:1365,height:900}],['mobile',{width:390,height:844}]])test(`keyboard, layout and visual sample: ${name}`,async ({page,browser})=>{
  await page.setViewportSize(viewport);const start=Date.now();await ready(page);const readiness=Date.now()-start;expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
  await page.locator('#budget').fill('79');await page.locator('#budget').press('Enter');await expect(page.locator('#overrun')).toBeVisible();await page.locator('#reset').focus();await page.keyboard.press('Enter');await expect(page.locator('#seed')).toBeFocused();await page.locator('#run').focus();await page.keyboard.press('Enter');await expect(page.locator('#counts')).toHaveText('3 selected · 1 excluded');
  mkdirSync('.cache/review',{recursive:true});await page.evaluate(()=>scrollTo(0,0));await page.screenshot({path:`.cache/review/${name}-entry.png`});await page.locator('#results').screenshot({path:`.cache/review/${name}-results.png`});writeFileSync(`.cache/review/${name}-observation.json`,JSON.stringify({browser:browser.version(),viewport,readiness_ms:readiness,fixture_units:4,scope:'One startup including network/worker/UI; not Rust CPU, retrieval or large-corpus benchmark'},null,2));
});
test.describe('without JavaScript',()=>{test.use({javaScriptEnabled:false});test('complete fixture, scope and independent downloads stay available',async ({page})=>{
  await page.goto('./');await expect(page.locator('#static-help')).toBeVisible();await expect(page.locator('.static-note')).toHaveCount(4);await expect(page.locator('#run')).toBeDisabled();await expect(page.locator('.static-note').nth(3)).toContainText('fixture:old');for(const path of ['fixture.json','source.zip','BUILD.md'])expect((await page.request.get('./'+path)).ok()).toBe(true);
});});
