import { useContext } from 'react'
import { UserContext } from './userContext'

const useUser = () => {
  const context = useContext(UserContext)
  return context
}
  
export default useUser