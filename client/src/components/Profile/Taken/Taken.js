import './Taken.css'
import { useLoaderData } from 'react-router-dom'

export async function loader({ params }) {
  const { userId } = params

  const request = {
    method: 'GET',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ userId })
  }

  // fetch(`http://localhost:8080/api/user/${userId}`, request)
  //   .then(response => response.json())
  //   .then(data => console.log(data))

  return { userId: params.userId }
}

const Taken = () => {
  const data = useLoaderData()
  
  return (
    <div>
      list
    </div>
  )
}

export default Taken
