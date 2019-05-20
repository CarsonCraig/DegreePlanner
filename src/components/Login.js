import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { GoogleLogin } from 'react-google-login'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'

const loginContainerStyle = {
  textAlign: 'center',
  fontFamily: 'Roboto, Arial, sans-serif',
  margin: '10px'
}

const googleLoginTextStyle = {
  fontFamily: 'Roboto, Arial, sans-serif',
  marginLeft: '10px'
}

export default class Login extends Component {
  constructor () {
    super()
    this.handleClick = this.handleClick.bind(this)
  }

  render () {
    const { errorMessage } = this.props

    return (
      <div style={loginContainerStyle}>
        <GoogleLogin
          clientId='988574817320-upn9d65cbmqvnol3h7fgro1cd7lo4l9h.apps.googleusercontent.com'
          onSuccess={this.handleClick}
          onFailure={this.handleClick}
          uxMode='popup'>
          <FontAwesomeIcon icon={['fab', 'google']} />
          <span style={googleLoginTextStyle}>Login with Google</span>
        </GoogleLogin>

        {errorMessage &&
          <p>{errorMessage}</p>
        }
      </div>
    )
  }

  handleClick (response) {
    this.props.onLoginClick(response)
  }
}

Login.propTypes = {
  onLoginClick: PropTypes.func.isRequired,
  errorMessage: PropTypes.string
}
