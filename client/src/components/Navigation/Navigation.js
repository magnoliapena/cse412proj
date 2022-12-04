import { Link } from 'react-router-dom'
import './Navigation.css'

const Navigation = () => {
  return (
    <div className='Row Navigation'>
      <Link className='Navigation-Link' to='/search'>Class Search</Link>
      <Link className='Navigation-Link' to='/login'>Log In</Link>
    </div>
  )
}

export default Navigation
