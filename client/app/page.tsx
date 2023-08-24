import { redirect } from 'next/navigation'

export default function Home() {
  // TODO: Handle authentication
  redirect('/login')

  return null
}
