import React, { Component } from 'react'

const messageStyle = {
  padding: '20px'
}

export default class Home extends Component {
  render () {
    return (
      <div style={messageStyle}>Welcome to UW CoursePlan! Please log in to get started.</div>
    )
  }
}
