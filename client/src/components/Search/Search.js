import './Search.css'
import { useState } from 'react'

const Search = () => {
  const [isAdvanced, setIsAdvanced] = useState(false)

  return (
    <div className='Search'>
      <h1>Class Search</h1>
        <div className='Row'>
          <div className="Column">
            <label>Term</label>
            <select className='TextInput'>
              <option>Spring 2023</option>
              <option>Fall 2022</option>
              <option>Summer 2022</option>
              <option>Sprint 2022</option>
              <option>Fall 2021</option>
              <option>Summer 2021</option>
            </select>
          </div>
          <div className="Column">
            <label>Subject</label>
            <input placeholder='Enter Subject' className='TextInput' />
          </div>
          <div className="Column">
            <label>Number</label>
            <input placeholder='Enter Number' className='TextInput' />
          </div>
        </div>
        {
          isAdvanced &&
          <>
          <div className='Row'>
            <div className="Column">
              <label>Location</label>
              <select className='TextInput'>
                <option>Select Location</option>
                <option>Tempe</option>
                <option>West</option>
                <option>Polytechnic</option>
                <option>Downtown Phoenix</option>
                <option>Online: iCourse</option>
              </select>
            </div>
            <div className="Column">
              <label>Session</label>
              <select className='TextInput'>
                <option>Select Session</option>
                <option>A</option>
                <option>B</option>
                <option>C</option>
              </select>
            </div>
            <div className="Column">
              <label>Instructor</label>
              <input className='TextInput' placeholder='Enter Instructor' />
            </div>
          </div>
          <div className='Row'>
            <div className="Column">
              <label>Number of Units</label>
              <select className='TextInput'>
                <option>Select Number</option>
                <option>1</option>
                <option>2</option>
                <option>3</option>
                <option>4</option>
                <option>5</option>
                <option>6</option>
              </select>
            </div>
            <div className="Column">
              <label>Days of the Week</label>
              <select className='TextInput'>
                <option>Select Days</option>
                <option>Monday</option>
                <option>Tuesday</option>
                <option>Wednesday</option>
                <option>Thursday</option>
                <option>Friday</option>
                <option>Saturday</option>
                <option>Sunday</option>
              </select>
            </div>
          </div>
          <div className='Row'>
            <div className="Column">
              <label>Class Status</label>
              <div className='Row'>
                <input type='radio' />
                <label>Open Classes</label>
              </div>
              <div className='Row'>
                <input type='radio' />
                <label>All Classes</label>
              </div>
            </div>
          </div>
          </>
        }
      <div className='Row' style={{ "marginTop": "30px" }}>        
        <button className='Button'>Search Classes</button>
        <button className='Button' onClick={() => setIsAdvanced(!isAdvanced)}>{isAdvanced ? 'Regular Search' : 'Advanced Search'}</button>
      </div>
    </div>
  )
}

export default Search
