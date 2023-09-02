import Link from 'next/link'

import { UserAuthForm } from '@/components/user-auth-form'

export default function AuthenticationPage() {
  return (
    <>
      <div className="container relative hidden min-h-screen min-w-screen flex-col items-center justify-center md:grid lg:px-0">
        <div className="lg:p-8">
          <div className="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]">
            <div className="flex flex-col space-y-2 text-center">
              <h1 className="text-2xl font-semibold tracking-tight">Create an account</h1>
              <p className="text-sm text-muted-foreground">
                Enter your email below to create your account
              </p>
            </div>
            <UserAuthForm />
          </div>
        </div>
      </div>
    </>
  )
}
