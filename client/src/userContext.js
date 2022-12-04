import { createContext, useState } from 'react'

export const UserContext = createContext()

export const UserProvider = (props) => {
  const [user, setUser] = useState(null)

  const value = {
    user, setUser
  }

  return (
    <UserContext.Provider value={value}>
      {props.children}
    </UserContext.Provider>
  )
}