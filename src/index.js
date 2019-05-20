import 'babel-polyfill'
import 'bootstrap/dist/css/bootstrap.min.css'
import React from 'react'
import { ApolloProvider } from 'react-apollo'
import { render } from 'react-dom'
import { Provider } from 'react-redux'
import { BrowserRouter } from 'react-router-dom'
import { createBrowserHistory } from 'history'
import { applyMiddleware, createStore } from 'redux'
import thunkMiddleware from 'redux-thunk'
import { client } from './api'
import App from './containers/App'
import appReducers from './reducers'

// FontAwesome
import { library } from '@fortawesome/fontawesome-svg-core'
import { fab } from '@fortawesome/free-brands-svg-icons'
library.add(fab)

const createStoreWithMiddleware = applyMiddleware(thunkMiddleware)(createStore)
export const store = createStoreWithMiddleware(appReducers)
const rootElement = document.getElementById('app')

export const history = createBrowserHistory()

class AppBrowserRouter extends BrowserRouter {
  constructor () {
    super()
    this.history = history
  }
}

render(
  <Provider store={store}>
    <ApolloProvider client={client}>
      <AppBrowserRouter>
        <App />
      </AppBrowserRouter>
    </ApolloProvider>
  </Provider>,
  rootElement
)
