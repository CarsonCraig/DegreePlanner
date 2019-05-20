import { ApolloClient } from 'apollo-client'
import { createHttpLink } from 'apollo-link-http'
import { setContext } from 'apollo-link-context'
import { InMemoryCache } from 'apollo-cache-inmemory'
import { onError } from 'apollo-link-error'
import { logoutUser } from './actions'
import { history, store } from './index'

export const BASE_URL = 'http://local.uwcourseplan.com:8000'

const httpLink = createHttpLink({
  uri: `${BASE_URL}/graphql`
})

const authLink = setContext((_, { headers }) => {
  const token = localStorage.getItem('access_token') || null
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : ''
    }
  }
})

const forbiddenCheckLink = onError(({ networkError }) => {
  if (networkError && [401, 403].includes(networkError.statusCode)) {
    store.dispatch(logoutUser())
    history.push('/')
  }
})

export const client = new ApolloClient({
  link: authLink
    .concat(forbiddenCheckLink)
    .concat(httpLink),
  cache: new InMemoryCache()
})
