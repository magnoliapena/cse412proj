import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import {
  createBrowserRouter,
  RouterProvider
} from "react-router-dom"

import App from './App'
import Login from './components/Login/Login'
import Profile from './components/Profile/Profile';
import Info from './components/Profile/Info/Info';
import Wishlist from './components/Profile/Wishlist/Wishlist';
import Taken from './components/Profile/Taken/Taken';
import CreateAccount from './components/CreateAccount/CreateAccount';
import Search from './components/Search/Search';

const router = createBrowserRouter([
  {
    path: '/',
    element: <App />,
    children: [
      {
        path: '/login',
        element: <Login />
      },
      {
        path: '/profile',
        element: <Profile />,
        children: [
          {
            path: '/profile/info',
            element: <Info />
          },
          {
            path: '/profile/wishlist',
            element: <Wishlist />
          },
          {
            path: '/profile/taken',
            element: <Taken />
          }
        ]
      },
      {
        path: '/create',
        element: <CreateAccount />
      },
      {
        path: '/search',
        element: <Search />
      }
    ]
  }
])

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
