import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { history } from '../index'

export default class Logout extends Component {
  render () {
    const { onLogoutClick } = this.props

    return (
      <button onClick={() => {
        onLogoutClick()
        history.push('/')
      }} className='btn btn-block btn-primary'>
        Log Out
      </button>
    )
  }
}

Logout.propTypes = {
  onLogoutClick: PropTypes.func.isRequired
}
