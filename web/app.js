const $ = id => document.getElementById(id);
$('static-help').hidden = true;
let worker, generation = 0, revision = 0, serial = 0, active = null;
let ready = false, busy = false, startupTimer, computeTimer, result = null, canonical = null;
function controls() {
  $('run').disabled = !ready || busy;
  for (const id of ['download','report-download','request-download']) $(id).disabled = !result;
}
function clearResults() {
  result = canonical = null; $('results').hidden = true;
  $('units').replaceChildren(); $('edges').replaceChildren(); $('metrics').textContent = '';
  controls();
}
function diagnostic(detail) {
  clearResults(); $('error').hidden = false;
  $('error-message').textContent = detail.message || 'The controls could not be evaluated.';
  $('diagnostic').textContent = JSON.stringify(detail,null,2);
}
function engineError(message, detail = '') {
  generation++; ready = busy = false; active = null;
  clearTimeout(startupTimer); clearTimeout(computeTimer); worker?.terminate(); worker = null;
  diagnostic({kind:'engine',message,...(detail ? {detail} : {})});
  $('status').textContent = 'Engine unavailable. Your controls are retained; fixture and source downloads still work.';
  $('retry').hidden = false; controls();
}
function invalidate() {
  revision++; clearResults(); $('error').hidden = true;
  if (ready) $('status').textContent = busy ? 'Controls changed. Waiting for the previous result.' : 'Controls changed. Explore again to refresh results.';
  // Startup/compute deadlines intentionally survive edits.
}
function textElement(tag, value, className = '') {
  const element = document.createElement(tag); element.textContent = value;
  if (className) element.className = className; return element;
}
function render(response) {
  $('counts').textContent = `${response.selected_count} selected · ${response.excluded_count} excluded`;
  $('selection-summary').textContent = `Seed: ${response.units[response.controls.seed].unit.id} · Estimated cost: ${response.pack.token_estimate} / ${response.controls.budget} · Representative: ${response.pack.representative_unit_id}`;
  $('overrun').hidden = !response.over_budget;
  $('overrun').textContent = `Seed overrun: ${response.budget_overrun} estimated token${response.budget_overrun === 1 ? '' : 's'} above your budget. The native kernel always retains the seed; this is not a strict budget guarantee.`;
  for (const item of response.units) {
    const card = document.createElement('article'); card.className = `unit ${item.selected ? 'selected' : 'excluded'}`;
    card.dataset.index = item.index; card.dataset.selected = String(item.selected);
    card.append(textElement('p',item.selected ? `Selected · ${item.reason}` : 'Excluded · not in this native pack','unit-state'));
    card.append(textElement('h3',item.unit.id)); card.append(textElement('p',item.unit.text));
    card.append(textElement('p',`Source: ${item.unit.source_id} · Estimated cost: ${item.unit.token_estimate} · Fixture relevance: ${item.unit.relevance}`,'hint'));
    const details = document.createElement('details'); details.append(textElement('summary','Hash and metadata'));
    const pre = textElement('pre',JSON.stringify({content_hash:item.unit.content_hash,metadata:item.unit.metadata},null,2)); pre.tabIndex = 0; details.append(pre); card.append(details);
    const seedButton = textElement('button','Use as starting note','secondary'); seedButton.type = 'button';
    seedButton.addEventListener('click',() => { $('seed').value = String(item.index); invalidate(); $('seed').focus(); });
    card.append(seedButton); $('units').append(card);
  }
  for (const edge of response.edges) $('edges').append(textElement('li',`${response.units[edge.from].unit.id} ↔ ${response.units[edge.to].unit.id} · ${edge.state}`,`edge-${edge.state}`));
  $('metrics').textContent = JSON.stringify({boundary_cut_edges:response.pack.boundary_cut_edges,boundary_conductance:response.pack.boundary_conductance,relevance_mean:response.pack.relevance_mean,quality_score:response.pack.quality_score},null,2);
  $('results').hidden = false;
}
function run() {
  if (!ready || busy) return;
  clearResults(); $('error').hidden = true;
  // Text is passed unchanged. Rust owns decimal admission; no JS numeric coercion.
  const request = JSON.stringify({seed:$('seed').value,budget:$('budget').value});
  if (request.length > 1024 || new TextEncoder().encode(request).length > 1024) {
    diagnostic({kind:'limit',message:'Request exceeds 1,024 UTF-8 bytes. Shorten the controls.'});
    $('status').textContent = 'Reduce the controls and try again.'; return;
  }
  busy = true; active = {id:++serial,revision,request}; controls();
  $('status').textContent = 'Selecting in Rust…';
  const id = active.id, epoch = generation;
  computeTimer = setTimeout(() => { if (generation === epoch && active?.id === id) engineError('Selection exceeded its deadline. Retry the engine, then explore your retained controls.'); },10000);
  try { worker.postMessage({type:'explore',id,request}); }
  catch (error) { engineError('The worker could not accept the controls.',String(error)); }
}
function start() {
  generation++; const epoch = generation;
  worker?.terminate(); clearTimeout(startupTimer); clearTimeout(computeTimer);
  ready = busy = false; active = null; clearResults(); $('error').hidden = true; $('retry').hidden = true;
  $('status').textContent = 'Loading Rust engine…';
  startupTimer = setTimeout(() => { if (epoch === generation && !ready) engineError('The Rust engine did not start before its deadline. Retry to load it again.'); },12000);
  try { worker = new Worker(new URL('./worker.js',import.meta.url),{type:'module'}); }
  catch (error) { engineError('The browser could not start the Rust worker.',String(error)); return; }
  worker.onerror = event => { event.preventDefault(); if (epoch === generation) engineError('The Rust worker failed. Retry with your current controls.',event.message); };
  worker.onmessage = ({data}) => {
    if (epoch !== generation) return;
    if (data.type === 'startup-error') { engineError('The Rust engine could not be downloaded or initialized.',data.message); return; }
    if (data.type === 'ready') {
      clearTimeout(startupTimer); ready = true; $('status').textContent = 'Rust engine ready. Explore your controls.'; controls();
      if (revision === 0) run(); return;
    }
    if (!active || data.id !== active.id) return;
    clearTimeout(computeTimer); const submitted = active; busy = false; active = null;
    if (data.type === 'compute-error') { engineError('The Rust worker could not finish selection.',data.message); return; }
    if (submitted.revision !== revision) { $('status').textContent = 'Controls changed. Explore again to refresh results.'; controls(); return; }
    let response;
    try { response = JSON.parse(data.canonical); } catch { engineError('The Rust worker returned an unreadable response.'); return; }
    if (!response.ok) { diagnostic(response.diagnostic); $('status').textContent = 'Fix the controls and explore again.'; controls(); return; }
    result = {...response,request:submitted.request}; canonical = data.canonical;
    render(response); $('status').textContent = 'Selection complete. Results reflect the current controls.'; controls();
  };
}
function download(name,value) {
  const url = URL.createObjectURL(new Blob([value],{type:'application/json;charset=utf-8'}));
  const link = document.createElement('a'); link.href = url; link.download = name; link.click(); setTimeout(() => URL.revokeObjectURL(url),1000);
}
$('explore-form').addEventListener('submit',event => {event.preventDefault();run();});
for (const id of ['seed','budget']) $(id).addEventListener('input',invalidate);
for (const button of document.querySelectorAll('[data-budget]')) button.addEventListener('click',() => { $('budget').value = button.dataset.budget; invalidate(); $('budget').focus(); });
$('reset').addEventListener('click',() => { $('seed').value = '0'; $('budget').value = '160'; invalidate(); $('seed').focus(); });
$('retry').addEventListener('click',start);
$('download').addEventListener('click',() => { if (result) download('mdcrop-fixture-pack.json',JSON.stringify(result.pack)); });
$('report-download').addEventListener('click',() => { if (result) download('mdcrop-fixture-report.json',canonical); });
$('request-download').addEventListener('click',() => { if (result) download('mdcrop-fixture-request.json',result.request); });
start();
