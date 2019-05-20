import React, { Component } from 'react'
import Term from './Term'
import { graphql } from 'react-apollo'
import PropTypes from 'prop-types'
import gql from 'graphql-tag'
import AddTerm from './AddTerm'
import RemoveTerm from './RemoveTerm'
import { Redirect } from 'react-router'

const timelineStyle = {
  display: 'flex',
  flexWrap: 'nowrap',
  justifyContent: 'center',
  marginTop: '25px'
}

const termFunctionsStyle = {
  display: 'flex',
  flexWrap: 'nowrap',
  justifyContent: 'center'
}

class Timeline extends Component {
  constructor (props) {
    super(props)
    this.state = {
      currentPlanId: 1,
      terms: [],
      lookupTable: {}
    }
  }

  updateCacheAfterAddTerm (store, addedTerm) {
    const data = store.readQuery({ query: GET_USER_TIMELINE })
    data.coursePlan.terms.push(addedTerm)
    store.writeQuery({ query: GET_USER_TIMELINE, data })
  }

  updateCacheAfterRemoveTerm (store, removedTerm) {
    const data = store.readQuery({ query: GET_USER_TIMELINE })
    data.coursePlan.terms = data.coursePlan.terms.filter((term) => { return term.id !== removedTerm.id })
    store.writeQuery({ query: GET_USER_TIMELINE, data })
  }

  updateCacheAfterAddCourse (store, addedCourse) {
    const data = store.readQuery({ query: GET_USER_TIMELINE })
    const newTerms = data.coursePlan.terms
    newTerms.forEach((term) => {
      if (term.id === addedCourse.termId) {
        term.courses.push(addedCourse)
      }
    })
    data.coursePlan.terms = newTerms
    store.writeQuery({ query: GET_USER_TIMELINE, data })
  }

  updateCacheAfterRemoveCourse (store, removedCourse) {
    const data = store.readQuery({ query: GET_USER_TIMELINE })
    const newTerms = data.coursePlan.terms
    newTerms.forEach((term) => {
      if (term.id === removedCourse.termId) {
        term.courses = term.courses.filter((term) => { return term.id !== removedCourse.id })
      }
    })
    data.coursePlan.terms = newTerms
    store.writeQuery({ query: GET_USER_TIMELINE, data })
  }

  render () {
    if (this.props.gqlData && this.props.gqlData.loading) {
      return <p>Loading...</p>
    }

    if (this.props.gqlData && this.props.gqlData.error) {
      return <Redirect to='/setup' />
    }

    this.state.currentPlanId = this.props.gqlData.coursePlan.id
    const termsList = this.props.gqlData.coursePlan.terms

    const newLookupTable = {}
    this.props.gqlData.coursePlan.terms.forEach((term) => {
      newLookupTable[term.name] = term.id
    })
    this.state.lookupTable = newLookupTable

    return (
      <div>
        <div style={timelineStyle}>
          {termsList.map((term) =>
            <Term key={term.id} id={term.id} name={term.name} courses={term.courses}
              updateCacheAfterAddCourse={this.updateCacheAfterAddCourse}
              updateCacheAfterRemoveCourse={this.updateCacheAfterRemoveCourse}
            />
          )}
        </div>
        <div style={termFunctionsStyle}>
          <AddTerm
            currentPlanId={this.state.currentPlanId}
            updateCacheAfterAddTerm={this.updateCacheAfterAddTerm}
          />
          <RemoveTerm
            updateCacheAfterRemoveTerm={this.updateCacheAfterRemoveTerm}
            lookupTable={this.state.lookupTable}
          />
        </div>
      </div>
    )
  }
}

const GET_USER_TIMELINE = gql`
query GetUserTimeline {
  coursePlan(default: true) {
    id
    terms {
      id
      name
      courses {
        id
        name
      }
    }
  }
}
`

Timeline.propTypes = {
  gqlData: PropTypes.any
}

export default graphql(GET_USER_TIMELINE, { name: 'gqlData' })(Timeline)
