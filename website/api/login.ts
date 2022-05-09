import axios from './axios'

export default async function login(username: string, password: string): Promise<string> {
  const request = axios.post("/auth/login", {
    username,
    password,
  })

  const response = request
    .then(response => {
      const { data } = response

      return data.LoggedIn.access_token
    })
    .catch(err => {
      if (err.response?.data?.error_message) {
        const error = err.response.data.error_message

        if (error.toString() == "[object Object]") {
          Object.keys(error).forEach(key => {
            throw new Error(key)
          })
        }

        throw new Error(error)
      }

      throw new Error(err)
    })

    return response
}
