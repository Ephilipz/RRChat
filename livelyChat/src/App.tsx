import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import './assets/styles/App.scss'

function App() {

  const [username, setUsername] = useState('');
  const navigate = useNavigate();

  function enterChat() {
    navigate(`/chat/${username}`);
  }

  return (
    <>
      <h1 className='mainTitle'>Welcome, <span className='light'>friend</span></h1>
      <h2 className='subTitle'>This is a lightweight chat room application developed with <strong>Rust</strong> and <strong>React</strong>. Have fun 🤠</h2>
      <div className='mainContainer'>
        <input placeholder='Your Name' onChange={(e) => setUsername(e.target.value)} value={username}/>
        <button className='btn enterChatBtn huge' onClick={enterChat}>Enter The Gates</button>
      </div>
    </>
  )
}

export default App
