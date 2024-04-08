import GeneralLoading from '@components/loading/GeneralLoading';
import {styled} from 'nativewind';
import React from 'react';
import {View} from 'react-native';

const StyledView = styled(View);

interface ScaffoldProps {
  topBar?: React.ReactElement<any, any>;
  children?: React.ReactElement<any, any>;
  bottomChild?: React.ReactElement<any, any>;
  loading?: boolean;
  background?: string;
}

const Scaffold: React.FC<ScaffoldProps> = ({
  topBar,
  children,
  bottomChild,
  loading,
  background,
}) => {
  return (
    <StyledView
      className="w-screen h-full relative"
      style={{backgroundColor: background}}>
      {topBar != null ? <StyledView>{topBar}</StyledView> : null}
      {children}
      {bottomChild != null ? (
        <StyledView className="absolute z-10 bottom-0 right-0 left-0 w-full h-auto">
          {bottomChild}
        </StyledView>
      ) : null}
      {loading ? <GeneralLoading loading={loading} /> : null}
    </StyledView>
  );
};

export default Scaffold;
