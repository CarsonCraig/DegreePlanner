import PropTypes from 'prop-types'
import React, { Component } from 'react'
import { connect } from 'react-redux'
import { Route, Switch, withRouter } from 'react-router-dom'
import { mapDispatchToProps } from '../actions'
import Header from '../components/Header'
import Home from '../components/Home'
import NotFound from '../components/NotFound'
import Setup from '../components/Setup'
import Timeline from '../components/Timeline'

class App extends Component {
  render () {
    const { dispatch, isAuthenticated, errorMessage } = this.props
    return (
      <div>
        <Header
          isAuthenticated={isAuthenticated}
          errorMessage={errorMessage}
          dispatch={dispatch} />
        <Switch>
          {!isAuthenticated &&
            <Route exact path='/' component={Home} />}

          {isAuthenticated && <Switch>
            <Route exact path='/' component={Timeline} />
            <Route path='/setup' component={Setup} />
          </Switch>}
          <Route component={NotFound} />
        </Switch>
      </div>
    )
  }
}

App.propTypes = {
  dispatch: PropTypes.func.isRequired,
  isAuthenticated: PropTypes.bool.isRequired,
  errorMessage: PropTypes.string
}

const mapStateToProps = (state) => {
  const { auth } = state
  const { isAuthenticated, errorMessage } = auth

  return {
    isAuthenticated,
    errorMessage
  }
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(App))
