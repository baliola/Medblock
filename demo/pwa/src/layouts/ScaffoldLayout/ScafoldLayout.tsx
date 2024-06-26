import React from 'react';

import GeneralLoading from '@/components/loading/GeneralLoading';

interface ScaffoldProps {
  topBar?: React.ReactElement<any, any>;
  children?: React.ReactElement<any, any>;
  bottomChild?: React.ReactElement<any, any>;
  loading?: boolean;
  style?: React.CSSProperties;
}

const Scaffold: React.FC<ScaffoldProps> = ({
  style,
  bottomChild,
  children,
  loading,
  topBar,
}) => {
  return (
    <div className="w-screen h-screen relative" style={style}>
      {topBar != null ? (
        <div className="fixed top-0 z-10 w-screen bg-white">{topBar}</div>
      ) : null}
      {children}
      {bottomChild != null ? (
        <div className="fixed bottom-0 w-full h-auto z-10">{bottomChild}</div>
      ) : null}
      {loading ? <GeneralLoading loading={loading} /> : null}
    </div>
  );
};

export default Scaffold;
