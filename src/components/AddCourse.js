import React, { Component } from 'react'
import { graphql } from 'react-apollo'
import PropTypes from 'prop-types'
import gql from 'graphql-tag'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import fontawesome from '@fortawesome/fontawesome'
import { faPlusSquare } from '@fortawesome/fontawesome-free-solid'

fontawesome.library.add(faPlusSquare)

const addBtnStyle = {
  color: 'white',
  backgroundColor: 'green',
  marginTop: '10px'
}

const addIconStyle = {
  marginRight: '5px'
}

const containterStyle = {
  display: 'flex',
  justifyContent: 'center',
  flexDirection: 'column',
  width: '125px'
}

const inputBoxStyle = {
  marginTop: '10px',
  textAlign: 'center'
}

class AddCourse extends Component {
  constructor (props) {
    super(props)
    this.state = {
      termId: props.termId,
      newName: ''
    }
  }

  render () {
    return (
      <div style={containterStyle}>
        <input
          className='mb2'
          value={this.state.newName}
          onChange={(val) => this.setState({ newName: val.target.value })}
          type='text'
          placeholder='new course'
          style={inputBoxStyle}
        />
        <button className='btn' style={addBtnStyle} onClick={() => this.addCourse()}>
          <FontAwesomeIcon style={addIconStyle} icon='plus-square' />
          Add Course
        </button>
      </div>
    )
  }

  async addCourse () {
    const { termId, newName } = this.state
    await this.props.gqlData({
      variables: {
        termId,
        newName
      },
      update: (store, { data: { createTermCourse } }) => {
        this.props.updateCacheAfterAddCourse(store, createTermCourse)
      }
    })
  }
}

const ADD_COURSE = gql`
mutation addCourse($termId: Int!, $newName: String!) {
  createTermCourse(termId: $termId, name: $newName) {
    id
    termId
    name
  }
}
`

AddCourse.propTypes = {
  gqlData: PropTypes.any,
  termId: PropTypes.number.isRequired,
  updateCacheAfterAddCourse: PropTypes.func.isRequired
}

export default graphql(ADD_COURSE, { name: 'gqlData' })(AddCourse)
