import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import init, { add } from "wasm-lib";
import './App.css';

function App() {
  const [todos, setTodos] = useState([
    {
      todo: 'Learn Rust',
      date: "12/2/4",
      completed:  false,
      color: 'border-purple-400'
    },
    {
      todo: 'Learn python',
      date: "12/8/4",
      completed:  false,
      color: 'border-purple-400',
    },
    {
      todo: 'Learn solidity',
      date: "12/1/4",
      completed:  false,
      color: 'border-green-400',
    },
    {
      todo: 'Learn wasm',
      date: "15/2/4",
      completed:  false,
      color: 'border-red-400'
    },
    {
      todo: 'Learn programming',
      date: "12/2/4",
      completed:  false,
      color: 'border-blue-400'
    },
    {
      todo: 'Learn something else ',
      date: "12/2/4",
      completed:  false,
      color: 'border-orange-400'
    },
    {
      todo: 'Learn Rust',
      date: "12/2/4",
      completed:  false,
      color: 'border-purple-400'
    }])
  const [ans, setAns] = useState(0)
  useEffect(() => {
    init().then(() => {
      setAns(add(1,1))
    })
    
  }, [])
  return (
    <div>
<div className="bg-black flex-row flex w-screen h-screen  text-white">
  <div className="w-full flex flex-col items-center justify-center ">
    <div>
      <h1 className="text-6xl font-bold">To Do List</h1>
      <p className='text-2xl'> simple todo list using react, wasm and rust </p>
    </div>
  </div>
  <div className="w-full  h-full flex flex-col justify-end border-white">
    {" "}
    {/* right side side */}
    <div className="flex flex-col bg-gray-800 rounded-md p-8 h-5/6">
      <div className="flex flex-row justify-between">
        {" "}
        {/* header */}
        <h1 className="font-bold text-4xl"> To Do </h1>
        <div>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
            <path strokeLinecap="round" strokeLinejoin="round" d="M10.5 6h9.75M10.5 6a1.5 1.5 0 11-3 0m3 0a1.5 1.5 0 10-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 01-3 0m3 0a1.5 1.5 0 00-3 0m-9.75 0h9.75" />
          </svg>
        </div>
      </div>
      <div className="flex justify-between">
        {/* Filter */}
        <p className='text-2xl font-semibold'>Today's tasks</p>
        <p className='text-2xl font-semibold'>All Tasks</p>
      </div>
      <div className="flex flex-col space-y-4">
        {" "}
        {/* To do lists */}
        {todos.map(todo => { 
          return (
            <div className="flex justify-around w-full p-2 items-center" key={Math.random() * 100}>
            <div className={`flex w-full border-l-8 pl-2 ${todo.color}`}>
              <div className="flex flex-col">
                <p className="font-bold text-2xl"> {todo.todo} </p>
                <p> {todo.date}</p>
              </div>
            </div>
            <div>
              <label className="relative inline-flex items-center mr-5 cursor-pointer">
                <input
                  type="checkbox"
                  defaultValue=""
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-gray-200 rounded-full peer dark:bg-gray-700 peer-focus:ring-4 peer-focus:ring-purple-300 dark:peer-focus:ring-purple-800 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-purple-600" />
              </label>
            </div>
          </div>
          )
        })}
      </div>
    </div>
  </div>
</div>

    </div>
  );
}

export default App;
