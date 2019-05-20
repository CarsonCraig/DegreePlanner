import React, { Component } from 'react'
import { graphql } from 'react-apollo'
import PropTypes from 'prop-types'
import gql from 'graphql-tag'
import { Button } from 'reactstrap'

const containterStyle = {
  display: 'flex',
  justifyContent: 'center',
  marginRight: '25px',
  flexDirection: 'column'
}

const inputBoxStyle = {
  marginBottom: '15px',
  textAlign: 'center'
}

class AddTerm extends Component {
  constructor (props) {
    super(props)
    this.state = {
      cpId: props.currentPlanId,
      newName: ''
    }
  }

  render () {
    return (
      <div style={containterStyle}>
        <div>
          <input
            className='mb2'
            value={this.state.newName}
            onChange={(val) => this.setState({ newName: val.target.value })}
            type='text'
            placeholder='new term to be added'
            style={inputBoxStyle}
          />
        </div>
        <Button color='primary' onClick={() => this.addTerm()}>Add Term</Button>
      </div>
    )
  }

  async addTerm () {
    const { cpId, newName } = this.state
    await this.props.gqlData({
      variables: {
        cpId,
        newName
      },
      update: (store, { data: { createTerm } }) => {
        this.props.updateCacheAfterAddTerm(store, createTerm)
      }
    })
  }
}

const ADD_TERM = gql`
mutation addTerm($cpId: Int!, $newName: String!) {
  createTerm(coursePlanId: $cpId, name: $newName) {
    id
    name
    courses {
      id
      name
    }
  }
}
`

AddTerm.propTypes = {
  gqlData: PropTypes.any,
  currentPlanId: PropTypes.number.isRequired,
  updateCacheAfterAddTerm: PropTypes.func.isRequired
}

export default graphql(ADD_TERM, { name: 'gqlData' })(AddTerm)
