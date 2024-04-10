import TextPrimary from '@components/text/TextPrimary';
import Scaffold from '@layouts/Scaffold';
import {styled} from 'nativewind';
import React from 'react';
import {View} from 'react-native';

const StyledView = styled(View);

const HistoryScreen = () => {
  return (
    <Scaffold>
      <StyledView className="flex-1 items-center justify-center">
        <TextPrimary text="Soon.." classStyle="text-gray-800 text-2xl" isBold />
      </StyledView>
    </Scaffold>
  );
};

export default HistoryScreen;
