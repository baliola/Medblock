import {styled} from 'nativewind';
import React from 'react';
import {Text, View} from 'react-native';
const StyledText = styled(Text);
const StyledView = styled(View);

const HomeScreen = () => {
  return (
    <StyledView className="flex-1 items-center justify-center">
      <Text className="text-red-500">sadjkalsdj</Text>
      <StyledText className="text-yellow-600 font-bold text-5xl">
        Try editing me!
      </StyledText>
    </StyledView>
  );
};

export default HomeScreen;
