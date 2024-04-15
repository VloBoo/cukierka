import React from 'react';
import ReactDOM from 'react-dom/client';

const elements = (
  <div className='name'>
    <h1>Hello</h1>
  </div>
)

const rootElement = document.getElementById("root");
if (rootElement === null) {
  console.error("Не удалось найти корневой элемент");
} else {
  const root = ReactDOM.createRoot(rootElement);
  root.render(elements);
}
