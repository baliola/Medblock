import { XMarkIcon } from '@heroicons/react/20/solid';
import Link from 'next/link';
import { useRouter } from 'next/router';

type sideNav = {
  closeNav: () => void;
};

const SideNav = ({ closeNav }: sideNav) => {
  const menu = [
    { id: 1, title: 'Home', path: '#' },
    {
      id: 2,
      title: `About Us`,
      path: '#about',
    },
    {
      id: 3,
      title: 'Product',
      path: '#product',
    },
  ];

  return (
    <div className="w-full h-full flex flex-col gap-6 justify-center items-center">
      <div className="w-full text-right flex justify-end pr-4">
        <button onClick={closeNav}>
          <XMarkIcon className="w-10 h-10 text-text-primary" />
        </button>
      </div>
      <ul className="flex flex-col gap-8 text-center justify-center items-center">
        {menu.map((menu) => (
          <li
            key={menu.id}
            className="flex justify-center text-center"
            onClick={closeNav}
          >
            <Link href={menu.path}>
              <a
                href={menu.path}
                className={`flex gap-4 items-center w-full capitalize h-8 hover:text-primary transition-all ease-in-out duration-300 relative after:absolute after:left-0 transition-colors-linier after:bottom-[-4px]  after:h-[3px] after:bg-primary after:rounded-full after:transition-[width] after:duration-300`}
              >
                {menu.title}
              </a>
            </Link>
          </li>
        ))}
      </ul>

      <a
        href="https://docs.dhatu.io/dhatu/"
        className="max-w-max"
        target="_blank"
        onClick={closeNav}
      >
        <button className=" max-w-max px-6 py-2 border border-primary rounded-xl bg-transparent text-primary hover:text-text-primary hover:bg-primary transition-all ease-in duration-300 ">
          Get Started
        </button>
      </a>
    </div>
  );
};

export default SideNav;
