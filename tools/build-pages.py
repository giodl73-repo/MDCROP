"""Build the bounded MDCROP demo, native references and inspectable source custody."""
import argparse, hashlib, html, json, os, shutil, subprocess, zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]

def capture(*args):return subprocess.check_output(args,cwd=ROOT).decode('utf-8').strip()
def run(*args):subprocess.run(args,cwd=ROOT,check=True)
def pin(path):return {'bytes':path.stat().st_size,'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,default=Path('dist/MDCROP'));a=p.parse_args()
 output=(ROOT/a.output).resolve();dist=(ROOT/'dist').resolve()
 if output==dist or not output.is_relative_to(dist):raise ValueError('Output must be a child of dist')
 if capture('wasm-bindgen','--version')!='wasm-bindgen 0.2.127':raise ValueError('Use wasm-bindgen 0.2.127')
 run('cargo','build','--locked','--release','--target','wasm32-unknown-unknown','-p','mdcrop-web','--lib','--features','wasm')
 run('cargo','build','--locked','-p','mdcrop-web','--bin','mdcrop-explore')
 metadata=json.loads(capture('cargo','metadata','--locked','--format-version','1','--filter-platform','wasm32-unknown-unknown','--features','mdcrop-web/wasm'))
 if output.exists():
  def linked(path):return path.is_symlink() or bool(getattr(path.lstat(),'st_file_attributes',0)&1024)
  if linked(output) or any(linked(p) for p in output.rglob('*')):raise ValueError('Linked output refused')
  shutil.rmtree(output)
 output.mkdir(parents=True)
 target=Path(metadata['target_directory']);run('wasm-bindgen',str(target/'wasm32-unknown-unknown/release/mdcrop_web.wasm'),'--target','web','--out-dir',str(output/'pkg'))
 cache=ROOT/'.cache';cache.mkdir(exist_ok=True)
 binary=target/'debug'/('mdcrop-explore.exe' if os.name=='nt' else 'mdcrop-explore')
 fixture_text=capture(str(binary),'--fixture');fixture=json.loads(fixture_text)
 page=(ROOT/'web/index.html').read_text(encoding='utf-8')
 options=''.join('<option value="'+str(i)+'">'+html.escape(u['id'])+'</option>' for i,u in enumerate(fixture['units']))
 cards=''.join('<article class="static-note"><h3>'+html.escape(u['id'])+'</h3><p>'+html.escape(u['text'])+'</p><p class="hint">Source: '+html.escape(u['source_id'])+' · Estimated cost: '+str(u['token_estimate'])+'</p><details><summary>Content hash</summary><code>'+html.escape(u['content_hash'])+'</code></details></article>' for u in fixture['units'])
 page=page.replace('{{SEED_OPTIONS}}',options).replace('{{FIXTURE_CARDS}}',cards)
 (output/'index.html').write_text(page,encoding='utf-8')
 for name in ('app.js','worker.js','style.css'):shutil.copyfile(ROOT/'web'/name,output/name)
 for name in ('LICENSE','BUILD.md'):shutil.copyfile(ROOT/name,output/name)
 shutil.copytree(ROOT/'tools/licenses/rust',output/'rust')
 (output/'fixture.json').write_text(fixture_text+'\n',encoding='utf-8',newline='')
 # Include notices for the portable adapter's actual resolved dependency closure,
 # including build-time proc macros, without adding consumer dependencies.
 packages={p['id']:p for p in metadata['packages']};nodes={n['id']:n for n in metadata['resolve']['nodes']}
 first=next(p['id'] for p in metadata['packages'] if p['name']=='mdcrop-web');pending=[first];visited=set();notices=[];dependencies=[]
 while pending:
  current=pending.pop()
  if current in visited:continue
  visited.add(current);pending.extend(nodes[current]['dependencies']);pkg=packages[current]
  if pkg['source'] is None:continue
  base=Path(pkg['manifest_path']).parent
  files=[]
  # Git workspace crates may inherit their grant from the workspace root.
  # Search only Cargo manifest ancestors, stopping at the checkout boundary.
  for directory in (base,*base.parents):
   if (directory/'Cargo.toml').is_file():
    files=sorted({f for pattern in ('LICENSE*','COPYRIGHT*','NOTICE*') for f in directory.glob(pattern) if f.is_file()})
    if files:break
   if (directory/'.git').exists():break
  if not files:raise ValueError('No package/workspace license notice found for '+pkg['name'])
  dependencies.append({'name':pkg['name'],'version':pkg['version'],'license':pkg['license'],'source':pkg['source']})
  notices.append('\n\n===== '+pkg['name']+' '+pkg['version']+' ('+str(pkg['license'])+') =====\n')
  for f in files:notices.append('\n--- '+f.name+' ---\n'+f.read_text(encoding='utf-8'))
 notice='MDCROP portable adapter dependency closure, including build-time proc macros.\nThe fixed-fixture adapter exports selection only; native filesystem/timing APIs remain unexposed. Original licenses follow.\nRust standard-library notices: rust/COPYRIGHT-library.html and rust/licenses/.\n'+''.join(notices)
 (output/'THIRD-PARTY-NOTICES.txt').write_text(notice,encoding='utf-8')
 dirty=bool(capture('git','status','--porcelain','--untracked-files=all'));commit=capture('git','rev-parse','HEAD')
 tracked=capture('git','ls-files','-z').split('\0')
 if dirty:
  with zipfile.ZipFile(output/'source.zip','w',compression=zipfile.ZIP_DEFLATED) as archive:
   for name in tracked:
    path=ROOT/name
    if path.is_file():
     if not path.resolve().is_relative_to(ROOT):raise ValueError('External source path refused')
     archive.write(path,name)
 else:run('git','archive','--format=zip','--output='+str(output/'source.zip'),'HEAD')
 with zipfile.ZipFile(output/'source.zip') as archive:
  for name in ('crates/mdcrop-web/src/lib.rs','crates/mdcrop-core/src/lib.rs','Cargo.lock','BUILD.md'):
   expected=(ROOT/name).read_bytes() if dirty else subprocess.check_output(['git','show',commit+':'+name],cwd=ROOT)
   if archive.read(name)!=expected:raise ValueError('Source ZIP differs from build source: '+name)
 for name in ('crates/mdcrop-web/src/lib.rs','crates/mdcrop-core/src/lib.rs','Cargo.lock'):
  dest=output/'source'/name;dest.parent.mkdir(parents=True,exist_ok=True)
  dest.write_bytes((ROOT/name).read_bytes() if dirty else subprocess.check_output(['git','show',commit+':'+name],cwd=ROOT))
 provenance={'schema':'mdcrop.pages-build.v1','commit':commit,'dirty':dirty,'source':'https://github.com/giodl73-repo/MDCROP',
   'rust':capture('rustc','--version'),'bindings':capture('wasm-bindgen','--version'),'dependencies':sorted(dependencies,key=lambda p:p['name']),
   'fixture_units':4,'delivery_budget_bytes':5_000_000}
 (output/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n',encoding='utf-8');(output/'.nojekyll').touch()
 cases=[]
 def case(name,request):
  path=cache/'request.json';path.write_text(request,encoding='utf-8',newline='')
  canonical=capture(str(binary),str(path));json.loads(canonical);cases.append({'name':name,'input':request,'canonical':canonical})
 for name,seed,budget in [('example','0','160'),('overrun','0','79'),('tight','0','80'),('wide','0','200'),('seed-one','1','160'),('seed-two','2','160'),('seed-noise','3','160'),('max','0','256')]:case(name,json.dumps({'seed':seed,'budget':budget}))
 for value in ['', '0','257','-1','+1','1.5','NaN','Infinity','0xA0','1e2',' 80 ']:case('invalid-budget-'+repr(value),json.dumps({'seed':'0','budget':value}))
 case('invalid-seed',json.dumps({'seed':'4','budget':'160'}))
 for name,request in [('missing','{}'),('wrong-type','{"seed":"0","budget":160}'),('unknown','{"seed":"0","budget":"160","graph":{}}'),('duplicate','{"seed":"0","seed":"1","budget":"160"}'),('malformed','[broken'),('oversize',' '*1025)]:case(name,request)
 case('boolean','{"seed":false,"budget":"160"}')
 case('null','{"seed":"0","budget":null}')
 (cache/'native-references.json').write_text(json.dumps(cases,indent=2)+'\n',encoding='utf-8')
 matrix=json.loads(capture(str(binary),'--matrix'));assert len(matrix)==1024
 (cache/'matrix-references.json').write_text(json.dumps(matrix)+'\n',encoding='utf-8')
 files={p.relative_to(output).as_posix():pin(p) for p in sorted(output.rglob('*')) if p.is_file()}
 size=sum(p['bytes'] for p in files.values())
 if size>=5_000_000:raise ValueError('Static demo exceeds the 5 MB delivery budget')
 (cache/'site-inventory.json').write_text(json.dumps({'commit':commit,'dirty':dirty,'bytes':size,'files':files},indent=2)+'\n')
 print(json.dumps({'output':str(output),'bytes':size,'files':len(files),'native_reference_cases':len(cases),'dirty':dirty}))

if __name__=='__main__':main()
