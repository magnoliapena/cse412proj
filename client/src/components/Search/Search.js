import './Search.css'
import { useState } from 'react'

const Search = () => {
  const [isAdvanced, setIsAdvanced] = useState(false)

  return (
    <div>
      <h1>Class Search</h1>
      <form>
        <div className="Column">
          <label>Term</label>
          <select />
        </div>
        <div className="Column">
          <label>Subject</label>
          <select />
        </div>
        <div className="Column">
          <label>Number</label>
          <select />
        </div>
        <div className="Column">
          <label>Keywords</label>
          <input />
        </div>
        {
          isAdvanced &&
          <>
          <div className="Column">
            <label>Location</label>
            <select />
          </div>
          <div className="Column">
            <label>Session</label>
            <select />
          </div>
          <div className="Column">
            <label>Level</label>
            <select />
          </div>
          <div className="Column">
            <label>Instructor</label>
            <input />
          </div>
          <div className="Column">
            <label>General Studies</label>
            <select />
          </div>
          <div className="Column">
            <label>College/School</label>
            <input />
          </div>
          <div className="Column">
            <label>Class Number</label>
            <select />
          </div>
          <div className="Column">
            <label>Number of Units</label>
            <input />
          </div>
          <div className="Column">
            <label>Days of the Week</label>
            <select />
          </div>
          <div className="Column">
            <label>Start Date</label>
            <input />
          </div>
          <div className="Column">
            <label>End Date</label>
            <input />
          </div>
          <div className="Column">
            <label>Class Status</label>
            <input />
          </div>
          <div className="Column">
            <label>Honors</label>
            <input />
          </div>
          </>
        }
        <button>Search Classes</button>
      </form>
      <button onClick={() => setIsAdvanced(!isAdvanced)}>{isAdvanced ? 'Regular Search' : 'Advanced Search'}</button>
    </div>
  )
}

export default Search
