import { Link, Outlet, useLoaderData } from 'react-router-dom'
import './Profile.css'

export async function loader({ params }) {
  return { userId: params.userId, view: params.view }
}

const Profile = () => {
  const data = useLoaderData()
  console.log(data);

  return (
    <div>
      <h1>@username</h1>
      <div className='Row'>
        <h2><Link to={`/profile/${data.userId}/info`}>Information</Link></h2>
        <h2><Link to={`/profile/${data.userId}/wishlist`}>Wishlist</Link></h2>
        <h2><Link to={`/profile/${data.userId}/taken`}>Taken</Link></h2>
      </div>
      <Outlet />
    </div>
  )
}

export default Profile
