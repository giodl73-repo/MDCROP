import init, { explore } from './pkg/mdcrop_web.js';
try {
  await init({module_or_path:new URL('./pkg/mdcrop_web_bg.wasm',import.meta.url)});
  self.postMessage({type:'ready'});
} catch (error) { self.postMessage({type:'startup-error',message:String(error)}); }
self.onmessage = ({data}) => {
  if (data.type !== 'explore') return;
  try { self.postMessage({type:'result',id:data.id,canonical:explore(data.request)}); }
  catch (error) { self.postMessage({type:'compute-error',id:data.id,message:String(error)}); }
};
