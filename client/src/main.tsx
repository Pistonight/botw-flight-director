import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import { parsePayload } from 'wasm';
import "@tensorflow/tfjs-backend-wasm";

declare const tflite: typeof import("@tensorflow/tfjs-tflite");
declare const tf: typeof import("@tensorflow/tfjs-core");

async function test() {
  await tf.ready();
  const img = tf.browser.fromPixels(document.getElementById('testimg') as HTMLImageElement);
  console.log(img.shape);
  const model = await tflite.loadTFLiteModel("botwqt.tflite")
}

test()

const decodeCString = (data: Uint8Array) => {
  const decoder = new TextDecoder('utf-8', { fatal: true });
  try {
    return decoder.decode(data);
  } catch (error) {
    console.error('Error decoding utf-8:', error);
    return '<invalid utf-8>';
  }
}

const websocket = new WebSocket('ws://localhost:8899');
websocket.onmessage = async (event) => {
  const blob = event.data as Blob;
  const buffer = new Uint8Array(await blob.arrayBuffer());
  try {

    const payload = parsePayload(buffer);
    if (payload.type === "Log") {
      console.log(decodeCString(payload.data));
    }
  } catch (error) {
    console.error(error);
  }
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
