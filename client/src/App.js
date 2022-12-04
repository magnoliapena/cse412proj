import { Outlet } from 'react-router-dom';
import './App.css'
import Navigation from './components/Navigation/Navigation';
import { UserProvider } from './userContext'

function App() {
  return (
    <UserProvider>
      <div className="App">
        <Navigation />
        <Outlet />
      </div>
    </UserProvider>
  );
}

export default App
