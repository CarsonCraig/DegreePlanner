import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { Jumbotron, Button } from 'reactstrap'
import { Link } from 'react-router-dom'

export default class NotFound extends Component {
  render () {
    const pathName = this.props.location.pathname.slice(1)
    return (
      <div>
        <Jumbotron>
          <h1 className='display-3'>404</h1>
          <p className='lead'>Unable to find page "{pathName}"</p>
          <Link to='/'><Button color='primary'>Homepage</Button></Link>
        </Jumbotron>
      </div>
    )
  }
}

NotFound.propTypes = {
  'location': PropTypes.object.isRequired
}
