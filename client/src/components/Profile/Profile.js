import { Link, Outlet, useLoaderData } from 'react-router-dom'
import './Profile.css'

export async function loader({ params }) {
  return { userId: params.userId }
}

const Profile = () => {
  const data = useLoaderData()

  return (
    <div className='Profile'>
      <h2>@{data.userId}</h2>
      <div className='Row'>
        <h3><Link className='Profile-Link' to={`/profile/${data.userId}/info`}>Information</Link></h3>
        <h3><Link className='Profile-Link' to={`/profile/${data.userId}/wishlist`}>Wishlist</Link></h3>
        <h3><Link className='Profile-Link' to={`/profile/${data.userId}/taken`}>Taken</Link></h3>
      </div>
      <Outlet />
    </div>
  )
}

export default Profile
