import { useRouter } from 'next/router';
import React, { FC, ReactElement, useState } from 'react';
import { File, History, Home, Settings } from 'solar-icon-set';

interface HomeLayoutProps {
  children: ReactElement;
}

type HomeNavigation = {
  id: number;
  label: string;
  path: string;
  icon: ReactElement;
};

export const HomeLayout: FC<HomeLayoutProps> = ({ children }) => {
  const router = useRouter();

  const [navigations, setNavigations] = useState<HomeNavigation[]>([
    {
      id: 1,
      label: 'Home',
      path: '/home',
      icon: (
        <Home
          color={router.pathname === '/home' ? '#242DA8' : '#374151'}
          size={18}
          iconStyle="Bold"
        />
      ),
    },
    {
      id: 2,
      label: 'File',
      path: '/file',
      icon: (
        <File
          color={router.pathname === '/file' ? '#242DA8' : '#374151'}
          size={18}
          iconStyle="Bold"
        />
      ),
    },
    {
      id: 3,
      label: 'History',
      path: '/history',
      icon: (
        <History
          color={router.pathname === '/history' ? '#242DA8' : '#374151'}
          size={18}
          iconStyle="Bold"
        />
      ),
    },
    {
      id: 4,
      label: 'Setting',
      path: '/setting',
      icon: (
        <Settings
          color={router.pathname === '/setting' ? '#242DA8' : '#374151'}
          size={18}
          iconStyle="Bold"
        />
      ),
    },
  ]);

  return (
    <div className="w-screen h-screen">
      {children}
      <div className="fixed bottom-0 left-0 z-50 w-full h-16 bg-white rounded-tl-xl rounded-tr-xl shadow-lg shadow-black">
        <div className="grid h-full max-w-lg grid-cols-4 mx-auto font-medium">
          {navigations.map((nav) => {
            return (
              <button
                key={nav.id}
                onClick={() => {
                  router.push(nav.path);
                }}
                type="button"
                className="inline-flex flex-col items-center justify-center px-5 hover:bg-blue-50  group"
              >
                {nav.icon}
                <span className="text-xs text-gray-800 mt-2 group-hover:text-primary-normal">
                  {nav.label}
                </span>
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
};

const getLayout = (page: ReactElement) => <HomeLayout>{page}</HomeLayout>;

export default getLayout;
