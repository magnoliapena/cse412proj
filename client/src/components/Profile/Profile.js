import { Link, Outlet } from 'react-router-dom'
import useUser from '../../useUser'
import './Profile.css'

const Profile = () => {
  const { user } = useUser()

  return (
    <div className='Profile'>
      <h2>@{user.username}</h2>
      <div className='Row'>
        <h3><Link className='Profile-Link' to={`/profile/info`}>Information</Link></h3>
        <h3><Link className='Profile-Link' to={`/profile/wishlist`}>Wishlist</Link></h3>
        <h3><Link className='Profile-Link' to={`/profile/taken`}>Taken</Link></h3>
      </div>
      <Outlet />
    </div>
  )
}

export default Profile
