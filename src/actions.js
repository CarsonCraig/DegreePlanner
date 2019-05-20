import axios from 'axios'
import { BASE_URL } from './api'

export const LOGIN_REQUEST = 'LOGIN_REQUEST'
export const LOGIN_SUCCESS = 'LOGIN_SUCCESS'
export const LOGIN_FAILURE = 'LOGIN_FAILURE'

const requestLogin = () => {
  return {
    type: LOGIN_REQUEST,
    isFetching: true,
    isAuthenticated: false
  }
}

const receiveLogin = (user) => {
  return {
    type: LOGIN_SUCCESS,
    isFetching: false,
    isAuthenticated: true,
    access_token: user
  }
}

const loginError = (message) => {
  return {
    type: LOGIN_FAILURE,
    isFetching: false,
    isAuthenticated: false,
    message
  }
}

export const LOGOUT_REQUEST = 'LOGOUT_REQUEST'
export const LOGOUT_SUCCESS = 'LOGOUT_SUCCESS'
export const LOGOUT_FAILURE = 'LOGOUT_FAILURE'

const requestLogout = () => {
  return {
    type: LOGOUT_REQUEST,
    isFetching: true,
    isAuthenticated: true
  }
}

const receiveLogout = () => {
  return {
    type: LOGOUT_SUCCESS,
    isFetching: false,
    isAuthenticated: false
  }
}

export function loginUser (response) {
  const { profileObj } = response
  const payload = { email: profileObj.email, googleId: profileObj.googleId, name: profileObj.name }

  return async dispatch => {
    dispatch(requestLogin())
    let apiResp
    try {
      apiResp = await axios.post(`${BASE_URL}/google_auth`, JSON.stringify(payload), { headers: {
        'Content-Type': 'application/json'
      } })
    } catch (error) {
      dispatch(loginError(JSON.stringify(error)))
    }

    console.log(apiResp)

    const { token } = apiResp.data
    localStorage.setItem('access_token', token)
    dispatch(receiveLogin({ access_token: token }))
  }
}

export function mapDispatchToProps (dispatch) {
  // TODO: get the server to verify token before trusting it for security reasons
  const accessToken = localStorage.getItem('access_token')
  if (accessToken) {
    dispatch(receiveLogin({ access_token: accessToken }))
  }
  return { dispatch }
}

export function logoutUser () {
  const token = localStorage.getItem('access_token') || ''
  return async dispatch => {
    dispatch(requestLogout())
    try {
      await axios.post(`${BASE_URL}/logout`, {}, { headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`
      } })
    } catch (ex) {
      // Ignore
    }
    localStorage.removeItem('id_token')
    localStorage.removeItem('access_token')
    dispatch(receiveLogout())
  }
}
