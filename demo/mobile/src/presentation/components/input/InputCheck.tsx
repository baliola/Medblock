import React, {useState} from 'react';

import {styled} from 'nativewind';
import {Linking, TouchableOpacity, View} from 'react-native';
import CheckBox from '@react-native-community/checkbox';
import Colors from '../../../constants/colors';
import TextPrimary from '../text/TextPrimary';
const StyledView = styled(View);
const StyledTouchableOpacity = styled(TouchableOpacity);

interface InputCheckProps {
  label: string;
  url?: string;
  onChange: (value: boolean) => void;
  value: boolean;
}

const InputCheck: React.FC<InputCheckProps> = ({
  label,
  onChange,
  value,
  url,
}) => {
  const [toggleCheckBox, setToggleCheckBox] = useState<boolean>(value);

  return (
    <StyledView className="flex flex-row items-center">
      <CheckBox
        disabled={false}
        value={toggleCheckBox}
        tintColors={{true: Colors.primary_normal, false: Colors.gray}}
        onValueChange={newValue => {
          setToggleCheckBox(newValue);
          onChange(newValue);
        }}
        onCheckColor={Colors.primary_normal}
        onTintColor={Colors.primary_normal}
        tintColor={Colors.primary_normal}
        onFillColor={Colors.primary_normal}
      />
      {url ? (
        <StyledTouchableOpacity
          onPress={() => {
            Linking.openURL(url);
          }}>
          <TextPrimary text={label} classStyle="text-gray-500 underline" />
        </StyledTouchableOpacity>
      ) : (
        <TextPrimary text={label} classStyle="text-gray-500" />
      )}
    </StyledView>
  );
};

export default InputCheck;
