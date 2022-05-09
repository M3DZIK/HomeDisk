import axios from './axios'

export default async function list(path: string, token: string): Promise<any> {
  const request = axios.post("/fs/list", {
    path
  }, {
    headers: {
      Authorization: `Bearer ${token}`,
    }
  })

  const response = request
    .then(response => {
      const { data } = response
      return data
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
