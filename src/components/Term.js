import React, { Component } from 'react'
import PropTypes from 'prop-types'
import RemoveCourse from './RemoveCourse'
import AddCourse from './AddCourse'

const listStyle = {
  listStyleType: 'none'
}

export default class Term extends Component {
  render () {
    return (
      <div>
        <ul style={listStyle}>
          <h3 style={{ textAlign: 'center' }}>{this.props.name}</h3>
          {this.props.courses.map((course) =>
            <RemoveCourse
              key={course.name}
              courseId={course.id}
              courseName={course.name}
              updateCacheAfterRemoveCourse={this.props.updateCacheAfterRemoveCourse}
            />
          )}
          <AddCourse
            termId={this.props.id}
            updateCacheAfterAddCourse={this.props.updateCacheAfterAddCourse}
          />
        </ul>
      </div>
    )
  }
}

Term.propTypes = {
  id: PropTypes.number.isRequired,
  name: PropTypes.string.isRequired,
  courses: PropTypes.array.isRequired,
  updateCacheAfterAddCourse: PropTypes.func.isRequired,
  updateCacheAfterRemoveCourse: PropTypes.func.isRequired
}
