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

class RemoveTerm extends Component {
  constructor (props) {
    super(props)
    this.state = {
      termName: ''
    }
  }

  render () {
    return (
      <div style={containterStyle}>
        <div>
          <input
            className='mb2'
            value={this.state.termName}
            onChange={(val) => this.setState({ termName: val.target.value })}
            type='text'
            placeholder='existing term to be removed'
            style={inputBoxStyle}
          />
        </div>
        <Button color='danger' onClick={() => this.removeTerm()}>Remove Term</Button>
      </div>
    )
  }

  async removeTerm () {
    const { termName } = this.state
    const termId = this.props.lookupTable[termName]
    if (typeof termId === 'undefined') return
    await this.props.gqlData({
      variables: {
        termId
      },
      update: (store, { data: { deleteTerm } }) => {
        this.props.updateCacheAfterRemoveTerm(store, deleteTerm)
      }
    })
  }
}

const REMOVE_TERM = gql`
mutation removeTerm ($termId: Int!) {
  deleteTerm(termId: $termId) {
    id
    name
  }
}
`

RemoveTerm.propTypes = {
  gqlData: PropTypes.any,
  updateCacheAfterRemoveTerm: PropTypes.func.isRequired,
  lookupTable: PropTypes.object.isRequired
}

export default graphql(REMOVE_TERM, { name: 'gqlData' })(RemoveTerm)
