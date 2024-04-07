import { createRootRoute, Link, Outlet } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'

export const Route = createRootRoute({
  component: () => Root(),
})

function Root() {
  return (
    <>
      <div className="p-2 flex gap-2">
        <Link to="/" className="[&.active]:font-bold">
          Home
        </Link>{' '}
        <Link to="/about" className="[&.active]:font-bold">
          About
        </Link>{' '}
        <Link to="/macs" className="[&.active]:font-bold">
          Macs
        </Link>{' '}
        <Link to="/spending" className="[&.active]:font-bold">
          Spending
        </Link>{' '}
        <Link to="/analytics" className="[&.active]:font-bold">
          Analytics
        </Link>
      </div>
      <hr />
      <Outlet />
      <TanStackRouterDevtools />
    </>
  );
}
