import React from 'react';
import ReactDOMClient from 'react-dom/client';
import Header from "./Header";
import './css/tooplate.css';
//import './css/fontawesome.css';
import './css/bootstrap.css';

const hello = 'Hello World';

const elements = (
  <div className="container">
    <Header />
    <h1>{hello}</h1>
  </div>
)

const rootElement = document.getElementById("root");
if (rootElement === null) {
  console.error("Не удалось найти корневой элемент");
} else {
  const root = ReactDOMClient.createRoot(rootElement);
  root.render(elements);
}
