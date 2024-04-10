import TextButton from '@components/button/TextButton';
import TextPrimary from '@components/text/TextPrimary';
import Strings from '@constants/strings';
import {styled} from 'nativewind';
import React from 'react';
import {View} from 'react-native';

const StyledView = styled(View);

const OnboardingDescription = () => {
  return (
    <StyledView className="flex flex-row flex-wrap justify-center px-6 items-center">
      <TextPrimary
        classStyle="text-gray-800 text-center text-xs"
        text={Strings.message.obBoardDesc}
      />
      <TextButton
        label={Strings.label.term}
        classStyle="text-primary-normal ml-1"
        isBold={false}
        onPress={() => {}}
      />
      <TextPrimary
        classStyle="text-gray-800 ml-1 text-xs"
        text={Strings.label.and}
      />
      <TextButton
        label={Strings.label.privacy}
        classStyle="text-primary-normal ml-1"
        isBold={false}
        onPress={() => {}}
      />
    </StyledView>
  );
};

export default OnboardingDescription;
