import {styled} from 'nativewind';
import React, {useState} from 'react';
import {
  KeyboardTypeOptions,
  NativeSyntheticEvent,
  TextInput,
  TextInputChangeEventData,
  TouchableOpacity,
  View,
} from 'react-native';
import Colors from '../../../constants/colors';
import {useTranslation} from 'react-i18next';
import TextPrimary from '@components/text/TextPrimary';

const StyledView = styled(View);
const StyledTextInput = styled(TextInput);

interface InputTextProps {
  id?: string;
  label?: string;
  placeholder?: string;
  classStyle?: string;
  isSecure?: boolean;
  value: string;
  onChange?: (e: NativeSyntheticEvent<TextInputChangeEventData>) => void;
  keyboardType?: KeyboardTypeOptions;
  prefix?: React.ReactElement<any, any>;
  suffix?: React.ReactElement<any, any>;
  error?: string;
  enable?: boolean;
  autoFocus?: boolean;
  outlined?: boolean;
}

const InputText: React.FC<InputTextProps> = ({
  id,
  label,
  placeholder,
  classStyle,
  isSecure = false,
  keyboardType,
  value,
  prefix,
  suffix,
  error,
  enable = true,
  autoFocus = false,
  onChange,
  outlined = false,
}) => {
  const [secure, setSecure] = useState<boolean>(true);
  const {t} = useTranslation('global');

  return (
    <StyledView className={classStyle}>
      {label != null ? (
        <TextPrimary text={label} classStyle="text-gray-600 mb-2" />
      ) : null}
      <StyledView
        style={{borderWidth: outlined ? 1 : 0}}
        className={
          'py-1 px-3 rounded-2xl flex flex-row justify-between items-center p-3 ' +
          `${outlined ? 'border-gray-200' : 'bg-slate-200'}`
        }>
        {prefix}
        <StyledTextInput
          id={id}
          className={'text-black flex-1 p-0 items-center'}
          cursorColor={Colors.primary_normal}
          placeholder={t(placeholder as string)}
          placeholderTextColor={Colors.gray}
          keyboardType={keyboardType}
          value={value}
          autoFocus={autoFocus}
          editable={enable}
          onChangeText={onChange as any}
          secureTextEntry={isSecure === true && secure}
          style={{fontFamily: 'Ubuntu-Regular', fontSize: 14}}
        />
        {isSecure ? (
          <TouchableOpacity
            onPress={() => {
              setSecure(!secure);
            }}>
            {/* <Icon
              name={secure ? 'eye' : 'eye-off'}
              color={Colors.gray_dark}
              size={20}
              style={{marginHorizontal: 4}}
            /> */}
          </TouchableOpacity>
        ) : null}
        {isSecure === false && suffix != null ? suffix : null}
      </StyledView>
      {error != null ? (
        <TextPrimary text={error} classStyle="text-red-500 text-xs mt-2" />
      ) : null}
    </StyledView>
  );
};

export default InputText;
