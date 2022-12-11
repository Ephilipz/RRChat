import React, { useEffect, useRef, useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import { toast, ToastContainer } from 'react-toastify';
import './assets/styles/ChatRoom.scss'
import 'react-toastify/dist/ReactToastify.css';
import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';
dayjs.extend(utc)

export interface Message {
  message: string,
  sender: string,
  senderId: string,
  time: string
}

export const ChatRoom = () => {

  import.meta.env.NODE_TLS_REJECT_UNAUTHORIZED = "0";

  const { username } = useParams();
  const navigate = useNavigate();

  const [roster, setRoster] = useState<Array<string>>([]);
  const [userMessage, setUserMessage] = useState<string>();
  const [userId, setUserId] = useState<string>('');
  const [messages, setMessageList] = useState<Array<Message>>([]);
  const socket = useRef<WebSocket>();

  const onMessageSent = () => {
    if (!userMessage)
      return;
    sendMessage(socket.current, userMessage);
    setUserMessage('');
  }

  const mapUsers = () => {
    return roster.map((user, index) => {
      return <span key={index} className='user'>{user}</span>
    })
  };

  const mapMessages = () => (
    messages.map((message, index) => {
      const isSelfMessage = message.senderId == userId;

      return <li className={'message chatBubble column' + (isSelfMessage ? ' self' : '')} key={index}>
        <span className='user'>
          {isSelfMessage ? 'you' : message.sender} @ {message.time}
        </span>
        {message.message}
      </li>

    }));

  useEffect(() => {
    if (!username) {
      navigate('/');
      return;
    }

    if (!socket.current?.OPEN)
      socket.current = openConnection(username, setMessageList, setRoster, setUserId);
  }, []);

  return (
    <div className='chatPageContainer'>
      <h1 className='chatRoomTitle'>Welcome to the room</h1>

      <div className='chatAndUsersContainer row md'>
        <div className='chatContainer double-column'>
          <div className='messageList'>
            {mapMessages()}
          </div>
          <div className='row stickyBottom'>
            <textarea className='userMessage' placeholder='type your message...' onChange={(e) => setUserMessage(e.target.value)} value={userMessage} />

            <button className='btn small green' onClick={(onMessageSent)}>Send</button>
          </div>

        </div>
        <div className='usersContainer column'>
          <h2 className='usersTitle'>Connected Users : {roster.length}</h2>
          <div className='column roster'>
            {mapUsers()}
          </div>
        </div>
      </div>

      <ToastContainer
        position="bottom-right"
        theme='dark' />
    </div>
  )
}

function openConnection(username: any, setMessageList: Function, setRoster: Function, setUserId: Function) {
  const serverURL = import.meta.env.VITE_SERVER_URL;
  let socket = new WebSocket(`${serverURL}/${username}`);

  socket.onopen = (event) => {
    onSocketOpen(event);
  }

  socket.onmessage = (event) => onMessageReceived(event, setMessageList, setRoster, setUserId);

  socket.onclose = (event) => onSocketClose(event);

  return socket;
}

function onSocketClose(event: CloseEvent): void {
  toast('session ended', { type: 'error' });
}

function onMessageReceived(event: any, setMessageList: Function, setRoster: Function, setUserId: Function): void {
  console.log('received message', event.data);
  const outerMessage = JSON.parse(event.data);
  const msgType = (outerMessage.msgType as string).toLowerCase();
  const { data } = outerMessage;

  switch (msgType) {
    case 'roster':
      const roster = JSON.parse(data.String);
      setRoster([...roster]);
      break;

    case 'userinfo':
      setUserId(data.String);
      break;

    case 'msg':
      const message = data.Message;
      const time = message.time.replace('UTC', '').trim();
      message.time = dayjs.utc(time).local().format('HH:mm');

      setMessageList((oldMessageList: Array<any>) =>
        [...oldMessageList, data.Message as Message]);
      break;

    default:
      break;
  }
}

function onSocketOpen(event: Event): any {
  toast('You are now Connected', { type: 'success' });
}

function sendMessage(socket: WebSocket | undefined, msg: string): any {
  if (!socket?.OPEN) {
    toast('Unable to send the message', { type: 'error' });
    return;
  }

  socket.send(msg);
}