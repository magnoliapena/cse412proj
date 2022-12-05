import './Search.css'
import { useState } from 'react'
import useUser from '../../useUser'

const Search = () => {
  const [isAdvanced, setIsAdvanced] = useState(false)
  const [term, setTerm] = useState('')
  const [subject, setSubject] = useState('')
  const [number, setNumber] = useState('')
  const [location, setLocation] = useState('')
  const [session, setSession] = useState('')
  const [instructor, setInstructor] = useState('')
  const [units, setUnits] = useState('')
  const [results, setResults] = useState(null)
  const { user } = useUser()

  const handleSubmit = () => {
    let query = ''

    if(term) query += `term=${term}&`
    if(subject && number) query += `course=${subject + number}&`
    if(location) query += `location=${location}&`
    if(session) query += `session=${session}&`
    if(instructor) query += `instructor=${instructor}&`
    if(units) query += `units=${units}&`

    const selectedDays = document.querySelectorAll('#select-days option:checked');
    let daysString = ''
    const daysValues = Array.from(selectedDays).map(el => daysString += el.value + ' ');
    if(daysString) query += `days=${daysString.slice(0, -1)}&`

    console.log(query)

    fetch(`http://98.161.210.47:8080/api/search_class?${query}`)
    .then(response => response.json())
    .then(resData => {
      setResults(resData)
    })
  }

  const handleAdd = (classid, term) => {
    const requestData = {
      userid: user.userid,
      classid,
      term
    }

    const request = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(requestData)
    }

    fetch('http://98.161.210.47:8080/api/user/add_to_wishlist', request)
      .then(response => response.json())
      .then(resData => {
        console.log(resData)
      })
  }

  console.log(results)

  return (
    <div className='Search'>
      <h1>Class Search</h1>
        <div className='Row'>
          <div className="Column">
            <label>Term</label>
            <select className='TextInput' onChange={event => setTerm(event.target.value)}>
              <option value=''>Select Term</option>
              <option value='2231'>Spring 2023</option>
              <option value='2227'>Fall 2022</option>
              <option value='2224'>Summer 2022</option>
              <option value='2221'>Spring 2022</option>
              <option value='2217'>Fall 2021</option>
              <option value='2214'>Summer 2021</option>
            </select>
          </div>
          <div className="Column">
            <label>Subject</label>
            <input placeholder='Enter Subject' className='TextInput' onChange={event => setSubject(event.target.value)}/>
          </div>
          <div className="Column">
            <label>Number</label>
            <input placeholder='Enter Number' className='TextInput' onChange={event => setNumber(event.target.value)}/>
          </div>
        </div>
        {
          isAdvanced &&
          <>
          <div className='Row'>
            <div className="Column">
              <label>Location</label>
              <select className='TextInput' onChange={event => setLocation(event.target.value)}>
                <option value=''>Select Location</option>
                <option value='tempe'>Tempe</option>
                <option value='west'>West</option>
                <option value='poly'>Polytechnic</option>
                <option value='downtown'>Downtown Phoenix</option>
                <option value='online'>Online: iCourse</option>
              </select>
            </div>
            <div className="Column">
              <label>Session</label>
              <select className='TextInput' onChange={event => setSession(event.target.value)}>
                <option value=''>Select Session</option>
                <option value='a'>A</option>
                <option value='b'>B</option>
                <option value='c'>C</option>
              </select>
            </div>
            <div className="Column">
              <label>Instructor</label>
              <input className='TextInput' placeholder='Enter Instructor' onChange={event => setInstructor(event.target.value)}/>
            </div>
          </div>
          <div className='Row' style={{ "alignItems": "flex-start" }}>
            <div className="Column">
              <label>Days of the Week</label>
              <select id='select-days' className='TextInput' multiple style={{ "height": "175px" }}>
                <option value='M'>Monday</option>
                <option value='T'>Tuesday</option>
                <option value='W'>Wednesday</option>
                <option value='Th'>Thursday</option>
                <option value='F'>Friday</option>
              </select>
            </div>
            <div className="Column">
              <label>Number of Units</label>
              <select className='TextInput' onChange={event => setUnits(event.target.value)}>
                <option>Select Number</option>
                <option value='1'>1</option>
                <option value='2'>2</option>
                <option value='3'>3</option>
                <option value='4'>4</option>
                <option value='5'>5</option>
                <option value='6'>6</option>
              </select>
            </div>
          </div>
          </>
        }
      <div className='Row' style={{ "marginTop": "30px" }}>        
        <button className='Button' onClick={handleSubmit}>Search Classes</button>
        <button className='Button' onClick={() => setIsAdvanced(!isAdvanced)}>{isAdvanced ? 'Regular Search' : 'Advanced Search'}</button>
      </div>
      {results && results.map((result, index) => {
        return (
          <div key={index} className='Row Search-Result'>
            <button className='Button' onClick={() => handleAdd(result.classid, result.term)}>Add</button>
            <p>{result.course}</p>
            <p>{result.days}</p>
            <p>{result.starttime} - {result.endtime}</p>
            <p>{result.instructor}</p>
            <p>{result.location}</p>
          </div>
        )
      })}
    </div>
  )
}

export default Search
