import React from 'react'
import ReactDOM from 'react-dom/client'
import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import App from './App'
import { ChatRoom } from './ChatRoom'
import './assets/styles/index.scss'
import VideoChatRoom from './VideoChatRoom'


const router = createBrowserRouter([
  {path: '/', element: <App />},
  {path: 'chat/:username', element: <ChatRoom />},
  {path: 'video/:username', element: <VideoChatRoom />},
])
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
)
