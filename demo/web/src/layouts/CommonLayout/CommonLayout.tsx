import Image from 'next/image';
import { FC, ReactElement, useEffect } from 'react';
import { AnimatePresence, motion } from 'framer-motion';

import styles from './CommonLayout.module.css';
import { useCentralStore } from '@/Store';
import Sidebar from '@/components/Sidebar/Sidebar';
import Navbar from '@/components/Navbar/Navbar';
import { useAuth } from '@/config/agent';
import { useRouter } from 'next/router';

interface CommonLayoutProps {
  children: ReactElement;
}

export const CommonLayout: FC<CommonLayoutProps> = ({ children }) => {
  const { isSidebarOpen, toggleSidebar, setIsSidebarOpen } = useCentralStore();
  const { authenticate, authenticated, identity } = useAuth();
  const router = useRouter();

  useEffect(() => {
    console.log('user not authorized');

    if (identity?.getPrincipal().toText() === undefined) {
      console.log('user not authorized');
      router.push('/auth/login');
    }
  }, []);
  return (
    <motion.div
      initial={{ opacity: 0, y: -20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
      className={`${isSidebarOpen ? 'overflow-hidden' : ''} h-screen`}
    >
      <AnimatePresence>
        {isSidebarOpen && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            onClick={() => setIsSidebarOpen(false)}
            className="bg-black/60 absolute top-0 left-0 md:hidden w-full h-screen z-20"
          />
        )}
      </AnimatePresence>

      <AnimatePresence>
        {isSidebarOpen && (
          <motion.div
            initial={{ x: '-100%' }}
            animate={{ x: 0 }}
            exit={{ x: '-100%' }}
            transition={{ duration: 0.3, type: 'spring', bounce: 0.25 }}
            className="absolute md:hidden z-30 top-0 left-0"
          >
            <Sidebar />
          </motion.div>
        )}
      </AnimatePresence>

      <div className="grid md:grid-cols-[240px_1fr] w-screen overflow-x-hidden">
        <div className="hidden md:block">
          <Sidebar />
        </div>
        <div className="flex">
          <div className="w-full overflow-x-auto max-w-[1440px] mx-auto">
            {/* <Navbar isOpen={isSidebarOpen} sidebarChange={toggleSidebar} /> */}
            {children}
          </div>
        </div>
      </div>
    </motion.div>
  );
};

const getLayout = (page: ReactElement) => <CommonLayout>{page}</CommonLayout>;

export default getLayout;
