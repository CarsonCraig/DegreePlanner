import React, { Component } from 'react'
import { graphql } from 'react-apollo'
import PropTypes from 'prop-types'
import gql from 'graphql-tag'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import fontawesome from '@fortawesome/fontawesome'
import { faMinusSquare } from '@fortawesome/fontawesome-free-solid'

fontawesome.library.add(faMinusSquare)

const listElementStyle = {
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  border: '1px solid grey',
  marginTop: '10px',
  padding: '10px'
}

const removeIconStyle = {
  margin: 0,
  color: 'grey'
}

const courseNameStyle = {
  marginLeft: '10px'
}

class RemoveCourse extends Component {
  constructor (props) {
    super(props)
    this.state = {
      courseId: props.courseId,
      courseName: props.courseName
    }
  }

  render () {
    return (
      <li key={this.state.courseName} style={listElementStyle}>
        <span style={courseNameStyle}>{this.state.courseName}</span>
        <FontAwesomeIcon
          size='lg' style={removeIconStyle} icon='minus-square'
          onClick={() => this.removeCourse()}
        />
      </li>
    )
  }

  async removeCourse () {
    const { courseId } = this.state
    await this.props.gqlData({
      variables: {
        courseId
      },
      update: (store, { data: { deleteTermCourse } }) => {
        this.props.updateCacheAfterRemoveCourse(store, deleteTermCourse)
      }
    })
  }
}

const REMOVE_COURSE = gql`
mutation removeCourse($courseId: Int!) {
  deleteTermCourse(termCourseId: $courseId) {
    id
    termId
    name
  }
}
`

RemoveCourse.propTypes = {
  gqlData: PropTypes.any,
  courseId: PropTypes.number.isRequired,
  courseName: PropTypes.string.isRequired,
  updateCacheAfterRemoveCourse: PropTypes.func.isRequired
}

export default graphql(REMOVE_COURSE, { name: 'gqlData' })(RemoveCourse)
