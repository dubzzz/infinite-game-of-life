var ht=(y,d)=>()=>(d||y((d={exports:{}}).exports,d),d.exports);var W=(y,d,a)=>{if(!d.has(y))throw TypeError("Cannot "+a)};var t=(y,d,a)=>(W(y,d,"read from private field"),a?a.call(y):d.get(y)),p=(y,d,a)=>{if(d.has(y))throw TypeError("Cannot add the same private member more than once");d instanceof WeakSet?d.add(y):d.set(y,a)},_=(y,d,a,R)=>(W(y,d,"write to private field"),R?R.call(y,a):d.set(y,a),a);var m=(y,d,a)=>(W(y,d,"access private method"),a);var ct=ht((lt,H)=>{(async()=>{var c,u,s,w,E,j,X,U,v,P;(function(){const l=document.createElement("link").relList;if(l&&l.supports&&l.supports("modulepreload"))return;for(const i of document.querySelectorAll('link[rel="modulepreload"]'))o(i);new MutationObserver(i=>{for(const e of i)if(e.type==="childList")for(const r of e.addedNodes)r.tagName==="LINK"&&r.rel==="modulepreload"&&o(r)}).observe(document,{childList:!0,subtree:!0});function n(i){const e={};return i.integrity&&(e.integrity=i.integrity),i.referrerPolicy&&(e.referrerPolicy=i.referrerPolicy),i.crossOrigin==="use-credentials"?e.credentials="include":i.crossOrigin==="anonymous"?e.credentials="omit":e.credentials="same-origin",e}function o(i){if(i.ep)return;i.ep=!0;const e=n(i);fetch(i.href,e)}})();const y="https://dubzzz.github.io/infinite-game-of-life/assets/gol_engine_bg-DQ3ZrjYK.wasm",d=async(l={},n)=>{let o;if(n.startsWith("data:")){const i=n.replace(/^data:.*?base64,/,"");let e;if(typeof Buffer=="function"&&typeof Buffer.from=="function")e=Buffer.from(i,"base64");else if(typeof atob=="function"){const r=atob(i);e=new Uint8Array(r.length);for(let h=0;h<r.length;h++)e[h]=r.charCodeAt(h)}else throw new Error("Cannot decode base64-encoded data URL");o=await WebAssembly.instantiate(e,l)}else{const i=await fetch(n),e=i.headers.get("Content-Type")||"";if("instantiateStreaming"in WebAssembly&&e.startsWith("application/wasm"))o=await WebAssembly.instantiateStreaming(i,l);else{const r=await i.arrayBuffer();o=await WebAssembly.instantiate(r,l)}}return o.instance.exports};let a;function R(l){a=l}const Q=typeof TextDecoder>"u"?(0,H.require)("util").TextDecoder:TextDecoder;let I=new Q("utf-8",{ignoreBOM:!0,fatal:!0});I.decode();let T=null;function Z(){return(T===null||T.byteLength===0)&&(T=new Uint8Array(a.memory.buffer)),T}function N(l,n){return l=l>>>0,I.decode(Z().subarray(l,l+n))}let B=null;function q(){return(B===null||B.byteLength===0)&&(B=new Int32Array(a.memory.buffer)),B}const F=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(l=>a.__wbg_universewasm_free(l>>>0));class L{static __wrap(n){n=n>>>0;const o=Object.create(L.prototype);return o.__wbg_ptr=n,F.register(o,o.__wbg_ptr,o),o}__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,F.unregister(this),n}free(){const n=this.__destroy_into_raw();a.__wbg_universewasm_free(n)}static new(){const n=a.universewasm_new();return L.__wrap(n)}add(n,o){const i=a.universewasm_add(this.__wbg_ptr,n,o);return L.__wrap(i)}next(){const n=a.universewasm_next(this.__wbg_ptr);return L.__wrap(n)}window(n,o,i,e){let r,h;try{const f=a.__wbindgen_add_to_stack_pointer(-16);a.universewasm_window(f,this.__wbg_ptr,n,o,i,e);var z=q()[f/4+0],g=q()[f/4+1];return r=z,h=g,N(z,g)}finally{a.__wbindgen_add_to_stack_pointer(16),a.__wbindgen_free(r,h,1)}}}function G(l,n){throw new Error(N(l,n))}URL=globalThis.URL;const x=await d({"./gol_engine_bg.js":{__wbindgen_throw:G}},y),J=x.memory,V=x.__wbg_universewasm_free,$=x.universewasm_new,tt=x.universewasm_add,et=x.universewasm_next,it=x.universewasm_window,nt=x.__wbindgen_add_to_stack_pointer,st=x.__wbindgen_free,ot=Object.freeze(Object.defineProperty({__proto__:null,__wbg_universewasm_free:V,__wbindgen_add_to_stack_pointer:nt,__wbindgen_free:st,memory:J,universewasm_add:tt,universewasm_new:$,universewasm_next:et,universewasm_window:it},Symbol.toStringTag,{value:"Module"}));R(ot);const k=[[{x:0,y:0}],[{x:-1,y:0},{x:0,y:0},{x:1,y:0}],[{x:-1,y:0},{x:0,y:0},{x:1,y:0},{x:2,y:0}],[{x:-1,y:0},{x:0,y:0},{x:1,y:0},{x:-1,y:1}],[{x:-1,y:0},{x:0,y:0},{x:1,y:0},{x:-2,y:1}],[{x:-1,y:0},{x:0,y:0},{x:1,y:0},{x:0,y:1}],[{x:-1,y:-1},{x:-1,y:0},{x:0,y:-1},{x:0,y:1},{x:1,y:0}],[{x:-1,y:-1},{x:-1,y:0},{x:0,y:-1},{x:0,y:1},{x:1,y:0},{x:1,y:1}],[{x:-1,y:0},{x:0,y:-1},{x:-1,y:1},{x:1,y:-1},{x:0,y:2},{x:2,y:0},{x:1,y:1}],[{x:0,y:-1},{x:1,y:0},{x:-1,y:1},{x:0,y:1},{x:1,y:1}]];class rt{constructor(n){p(this,E);p(this,X);p(this,v);p(this,c,void 0);p(this,u,void 0);p(this,s,void 0);p(this,w,void 0);_(this,c,document.createElement("canvas")),t(this,c).style.position="fixed",t(this,c).style.left="0",t(this,c).style.right="0",t(this,c).style.height="100vh",t(this,c).style.width="100vw",t(this,c).className="mode-move",n.appendChild(t(this,c)),m(this,E,j).call(this),_(this,u,L.new()),_(this,u,t(this,u).add(10,10)),_(this,u,t(this,u).add(11,10)),_(this,u,t(this,u).add(12,10)),_(this,s,{x:0,y:0,zoom:20}),_(this,w,[]);let o=0,i;t(this,c).addEventListener("mousedown",e=>{if(e.button===0){if(_(this,w,[]),e.ctrlKey){const r=m(this,v,P).call(this,{x:e.clientX,y:e.clientY});for(const h of k[o])_(this,u,t(this,u).add(r.y+h.y,r.x+h.x));this.redrawScene(),i={mode:"draw"};return}i={mode:"move",initialOrigin:t(this,s),initialMousePosition:{x:e.clientX,y:e.clientY}}}}),t(this,c).addEventListener("mousemove",e=>{if(i===void 0){if(_(this,w,[]),e.ctrlKey){const r=m(this,v,P).call(this,{x:e.clientX,y:e.clientY});for(const h of k[o])t(this,w).push({x:r.x+h.x,y:r.y+h.y});this.redrawScene();return}return}switch(i.mode){case"move":{const r=e.clientX-i.initialMousePosition.x,h=e.clientY-i.initialMousePosition.y;_(this,s,{...t(this,s),x:i.initialOrigin.x-r,y:i.initialOrigin.y-h}),this.redrawScene();break}case"draw":{const r=m(this,v,P).call(this,{x:e.clientX,y:e.clientY});for(const h of k[o])_(this,u,t(this,u).add(r.y+h.y,r.x+h.x));this.redrawScene();break}}}),t(this,c).addEventListener("mouseup",e=>{e.button===0&&(i=void 0)}),t(this,c).addEventListener("wheel",e=>{e.preventDefault();const r=Math.sign(e.deltaY);if(e.ctrlKey&&i===void 0||(i==null?void 0:i.mode)==="draw"){o=(o+r+k.length)%k.length;const z=m(this,v,P).call(this,{x:e.clientX,y:e.clientY});for(const g of k[o])t(this,w).push({x:z.x+g.x,y:z.y+g.y});this.redrawScene();return}const h=Math.max(1,t(this,s).zoom-r);_(this,s,{x:h/t(this,s).zoom*(t(this,s).x+e.clientX)-e.clientX,y:h/t(this,s).zoom*(t(this,s).y+e.clientY)-e.clientY,zoom:h}),this.redrawScene()}),document.addEventListener("keyup",e=>{!e.ctrlKey&&t(this,w).length!==0&&(_(this,w,[]),this.redrawScene())}),new ResizeObserver(()=>{m(this,E,j).call(this),this.redrawScene()}).observe(t(this,c)),m(this,X,U).call(this)}redrawScene(){const n=t(this,c).width,o=t(this,c).height,i=Math.floor(t(this,s).x/t(this,s).zoom),e=Math.ceil((t(this,s).x+n-1)/t(this,s).zoom)-i+1,r=Math.floor(t(this,s).y/t(this,s).zoom),h=Math.ceil((t(this,s).y+o-1)/t(this,s).zoom)-r+1,z=t(this,u).window(r,i,r+h-1,i+e-1),g=t(this,c).getContext("2d"),f=g.getImageData(0,0,n,o);for(let O=0;O!==h;++O)for(let Y=0;Y!==e;++Y){const K=Y+O*e,D=z[K]==="*";for(let M=0;M!==t(this,s).zoom;++M){const A=O*t(this,s).zoom+M;if(A>=o)break;for(let S=0;S!==t(this,s).zoom;++S){const b=Y*t(this,s).zoom+S;if(b>=n)break;const C=(b+A*n)*4;f.data[C+0]=0,f.data[C+1]=D?255:0,f.data[C+2]=0,f.data[C+3]=255}}}for(const O of t(this,w)){const{x:Y,y:K}=O;for(let D=0;D!==t(this,s).zoom;++D){const M=-Math.floor(t(this,s).y)+K*t(this,s).zoom+D;if(!(M<0||M>=o))for(let A=0;A!==t(this,s).zoom;++A){const S=-Math.floor(t(this,s).x)+Y*t(this,s).zoom+A;if(S<0||S>=n)continue;const b=(S+M*n)*4;f.data[b+0]=0,f.data[b+1]=Math.max(f.data[b+1],127),f.data[b+2]=0,f.data[b+3]=255}}}g.putImageData(f,0,0)}}c=new WeakMap,u=new WeakMap,s=new WeakMap,w=new WeakMap,E=new WeakSet,j=function(){const n=t(this,c).getBoundingClientRect();t(this,c).height=n.height,t(this,c).width=n.width},X=new WeakSet,U=function(){this.redrawScene(),setTimeout(()=>{_(this,u,t(this,u).next()),m(this,X,U).call(this)},250)},v=new WeakSet,P=function(n){const o=Math.floor((t(this,s).x+n.x)/t(this,s).zoom),i=Math.floor((t(this,s).y+n.y)/t(this,s).zoom);return{x:o,y:i}};function at(l){new rt(l)}at(document.querySelector("#app"))})()});export default ct();
