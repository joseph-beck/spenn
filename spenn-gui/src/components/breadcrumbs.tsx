import { Link } from "@tanstack/react-router";

export interface Breadcrumb {
  name: string;
  href: string;
}

export const BreadcrumbsComponent = (props: {
  breadcrumbs: Breadcrumb[],
  current?: Breadcrumb,
}): JSX.Element => {
  return (
    <>
      <ol className="inline-flex">
        {
          props.breadcrumbs.map((crumb: Breadcrumb, index: number) => (
            <li key={index} className="pr-2">
              <Link to={crumb.href}>{crumb.name} /</Link>
            </li>
          ))
        }
        {
          props.current &&
          <li>
            <Link className="text-gray-500" disabled={true} to={props.current.href}>{props.current.name}</Link>
          </li>
        }
      </ol>
    </>
  );
};
