import React, { Component } from 'react'
import PropTypes from 'prop-types'
import Login from './Login'
import Logout from './Logout'
import { loginUser, logoutUser } from '../actions'
import { Query } from 'react-apollo'
import { Navbar, Nav, UncontrolledDropdown, DropdownToggle, DropdownMenu, DropdownItem } from 'reactstrap'
import gql from 'graphql-tag'
import { Link } from 'react-router-dom'

const navbarStyle = {
  paddingLeft: '10px',
  paddingTop: '0px',
  paddingBottom: '0px',
  paddingRight: '10px'
}

const navbarBrandStyle = {
  fontSize: '34px',
  fontFamily: 'Roboto, Arial, sans-serif',
  padding: '10px',
  borderRight: '1px solid #323232'
}

const dropdownItemStyle = {
  padding: '0px'
}

const GET_USERNAME = gql`
{
  me {
    name
  }
}
`

export default class Header extends Component {
  render () {
    const { dispatch, errorMessage, isAuthenticated } = this.props

    return (

      <div>

        <Navbar style={navbarStyle} color='dark' dark expand='md'>
          <Link to='/' className='navbar-brand' style={navbarBrandStyle}>UW CoursePlan</Link>
          <Nav className='ml-auto' navbar>

            {!isAuthenticated &&
              <Login
                errorMessage={errorMessage}
                onLoginClick={(creds) => dispatch(loginUser(creds))}
              />
            }
            {isAuthenticated &&

              <div>
                <UncontrolledDropdown setActiveFromChild>
                  <Query query={GET_USERNAME}>
                    {({ loading, error, data }) => {
                      if (loading) return <p>Loading...</p>
                      if (error) return <p>Error!</p>

                      return (
                        <DropdownToggle style={{ fontSize: '20px' }} tag='a' className='nav-link' caret>
                          {data.me.name}
                        </DropdownToggle>
                      )
                    }}
                  </Query>
                  <DropdownMenu right >
                    <DropdownItem tag='a' style={dropdownItemStyle} active> <Logout onLogoutClick={() => dispatch(logoutUser())} /></DropdownItem>
                  </DropdownMenu>
                </UncontrolledDropdown>
              </div>
            }
          </Nav>
        </Navbar>
      </div>
    )
  }
}

Header.propTypes = {
  dispatch: PropTypes.func.isRequired,
  isAuthenticated: PropTypes.bool.isRequired,
  errorMessage: PropTypes.string
}
