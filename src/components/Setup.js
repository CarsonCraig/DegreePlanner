import React, { Component } from 'react'
import { PropTypes } from 'prop-types'
import { Button, Form, FormGroup, Label, Input,
  UncontrolledDropdown, DropdownToggle, DropdownMenu, DropdownItem } from 'reactstrap'
import { graphql } from 'react-apollo'
import gql from 'graphql-tag'
import { client } from '../api'
import { history } from '../index'

const setupContainerStyle = {
  margin: '15px',
  textAlign: 'center'
}

const startFromScratchButtonStyle = {
  marginTop: '15px'
}

const italic = {
  fontStyle: 'italic'
}

const dropdownStyle = {
  marginTop: '12px',
  marginBottom: '12px'
}

const invisibleDropdownStyle = {
  display: 'none'
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

const CREATE_COURSE_PLAN = gql`
mutation addCoursePlan($cpInput: CreateCoursePlanInput!) {
  createCoursePlan(params: $cpInput) {
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

class Setup extends Component {
  constructor (props) {
    super(props)
    this.state = {
      selectedProgramObj: null,
      selectedYear: null,
      transcriptText: '',
      // TODO: We will need to massage the following data structure into the format
      // needed to line up with the Graph API request/response we'll make for programs
      programs: {
        'Engineering': {
          'Software Engineering': {
            years: {
              '2018-2019': 'uw-software-engineering_2018-2019_stream-8'
            },
            name: 'Software Engineering'
          }
        }
      }
    }

    this.handleTranscriptTextChange = this.handleTranscriptTextChange.bind(this)
    this.uploadTranscriptData = this.uploadTranscriptData.bind(this)
    this.buildPlanForProgram = this.buildPlanForProgram.bind(this)
  }

  async goToCoursePlan () {
    const { gqlData } = this.props
    if (gqlData && gqlData.error) {
      await client.mutate({
        variables: { cpInput: {} },
        mutation: CREATE_COURSE_PLAN
      })
    }
    history.push('/')
  }

  buildProgramDropdown () {
    const result = []
    for (const department in this.state.programs) {
      if (!this.state.programs.hasOwnProperty(department)) {
        continue
      }
      result.push(
        <DropdownItem key={department + 'header'} header>{department}</DropdownItem>
      )
      const programs = this.state.programs[department]
      for (const programName in programs) {
        if (!programs.hasOwnProperty(programName)) {
          continue
        }
        const programObj = programs[programName]
        result.push(
          <DropdownItem onClick={() => {
            this.setState({
              selectedProgramObj: programObj,
              selectedYear: null
            })
          }} key={programName}>{programName}</DropdownItem>
        )
      }
      result.push(
        <DropdownItem key={department + 'divider'} divider />
      )
    }
    result.pop() // Remove last divider
    return result
  }

  buildYearsDropdown () {
    if (!this.state.selectedProgramObj) {
      return []
    }
    return Object.entries(this.state.selectedProgramObj.years).map((schoolYear) => {
      return <DropdownItem onClick={() => {
        this.setState({ selectedYear: schoolYear })
      }} key={schoolYear[1]}>{schoolYear[0]}</DropdownItem>
    })
  }

  handleTranscriptTextChange (event) {
    this.setState({ transcriptText: event.target.value })
  }

  async uploadTranscriptData () {
    // Unpack the transcriptText from the state object for this React Component.
    const { transcriptText } = this.state

    var regex = /\sLevel:\s(..)\sCourse\s*(.*?)\sTerm GPA/g

    var textWithoutNewlines = transcriptText.replace(/\n/g, ' ')

    var terms = []
    do {
      var termMatch = regex.exec(textWithoutNewlines)
      if (termMatch) {
        var termName = termMatch[1]
        var courseMatches = termMatch[2].match(/[^ ]+ [^ ]+/g)

        var courses = []
        courseMatches.forEach(function (courseName) {
          courses.push({ name: courseName })
        })

        terms.push({ name: termName, courses: courses })
      }
    } while (termMatch)

    var coursePlan = { terms: terms }
    console.log(JSON.stringify(coursePlan, null, 4))

    // TODO(Carson): for now this just prints out the user's transcript text. What we
    // need is an algorithm to parse it and populate the data structure `coursePlan`
    // below based on the information found in the transcriptText. So basically instead
    // of having the `coursePlan` constant hard-coded like we have below, we want to create/fill
    // it from the transcript.
    // Here is a sample UW undergraduate transcript text:
    // https://gist.github.com/kennethsinder/1cd5131749f6d09836d9e7cede5b37ac

    // TODO(ksinder): Link up the mutation(s) for this. Once the data
    // structure has been constructed by the above code, what remains is to pass
    // it to the back-end as a mutation to create and populate the course plan
    // (for the currently authenticated user)>

    try {
      const result = await client.mutate({
        variables: { cpInput: { 'transcript': JSON.stringify(coursePlan) } },
        mutation: CREATE_COURSE_PLAN
      })
      console.log(result)
    } catch (exc) {
      console.log('Exception: ', exc)
      console.log('This is because the non-empty course plan mutation is not ready yet!')
    }
  }

  async buildPlanForProgram () {
    try {
      // Build a course plan for the internal program-year string representation that corresponds
      // to what the user selected in the 2 dropdowns.
      const result = await client.mutate({
        variables: { cpInput: { 'program': this.state.selectedYear[1] } },
        mutation: CREATE_COURSE_PLAN
      })
      console.log(result)
    } catch (exc) {
      console.log('Exception: ', exc)
      console.log('This is because the non-empty course plan mutation is not ready yet!')
    }

    // Go to the course plan page:
    history.push('/')
  }

  render () {
    return (
      <div style={setupContainerStyle}>
        <div>
          <span>
            <strong>Option 1:</strong>
          </span>
          <span> Start with an <span style={italic}>empty</span> course plan and add terms and courses yourself.</span>
        </div>
        <Button color='primary' style={startFromScratchButtonStyle} onClick={() => this.goToCoursePlan()}>
          Go to course plan
        </Button>

        <hr />

        <div>
          <span>
            <strong>Option 2:</strong>
          </span>
          <span> Select your program and start year to get a quick start with all the courses you need to take.</span>
        </div>
        <UncontrolledDropdown style={dropdownStyle}>
          <DropdownToggle caret>
            {!this.state.selectedProgramObj ? 'Program Name' : this.state.selectedProgramObj.name}
          </DropdownToggle>
          <DropdownMenu>
            {this.buildProgramDropdown()}
          </DropdownMenu>
        </UncontrolledDropdown>
        <UncontrolledDropdown
          style={!this.state.selectedProgramObj ? invisibleDropdownStyle : dropdownStyle}>
          <DropdownToggle caret>
            {!this.state.selectedYear ? 'Starting Year' : this.state.selectedYear[0]}
          </DropdownToggle>
          <DropdownMenu>
            {this.buildYearsDropdown()}
          </DropdownMenu>
        </UncontrolledDropdown>
        <Button color='primary' style={startFromScratchButtonStyle}
          disabled={!this.state.selectedYear && !this.stateSelectedProgram}
          onClick={this.buildPlanForProgram}>
          Load my program calendar
        </Button>

        <hr />

        <div>
          <span>
            <strong>Option 3:</strong>
          </span>
          <span> Copy your transcript from Quest to load up all the courses you&#8217;ve already taken for a quicker start.</span>
        </div>
        <Form>
          <FormGroup>
            <Label for='transcriptText'>Copy and paste your entire transcript here:</Label>
            <Input type='textarea' name='transcriptText' id='transcriptText'
              value={this.state.transcriptText} onChange={this.handleTranscriptTextChange} />
          </FormGroup>
        </Form>
        <Button color='primary'
          style={startFromScratchButtonStyle}
          disabled={!this.state.transcriptText}
          onClick={this.uploadTranscriptData}>
          Load my transcript
        </Button>
      </div>
    )
  }
}

Setup.propTypes = {
  gqlData: PropTypes.any
}

export default graphql(GET_USER_TIMELINE, { name: 'gqlData' })(Setup)
