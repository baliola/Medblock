import { Flex, FormControl, FormErrorMessage, FormLabel, Icon, Input, InputProps, Text } from "@chakra-ui/react";

import { Field, useFormikContext } from "formik";
import { EMR } from "@/libs/yup/emr";
import { vitalSignHeader, vitalSignInput } from "@/constants/contents/dashboard/emr/form";

interface InputFrameProps {
  label: string;
  icon: React.ElementType;
  name: string;
  placeholder: string;
  unit: string;
}

const InputFrame: React.FC<InputFrameProps> = ({
  label, icon, name, placeholder, unit
}) => {
  const { errors, touched } = useFormikContext<EMR>();
  const valueName = name as keyof EMR;

  const hasError = !!errors[valueName] && touched[valueName];

  const selectorBehaviorInputProps: InputProps = {
    boxShadow: 'none',
    outline: 'none',
    border: 'none'
  };

  const defaultBehaviorInputProps: InputProps = {
    ...selectorBehaviorInputProps,
    _focus: { ...selectorBehaviorInputProps },
    _hover: { ...selectorBehaviorInputProps },
    _active: { ...selectorBehaviorInputProps },
    _invalid: { ...selectorBehaviorInputProps },
  }

  return (
    <FormControl
      isRequired
      isInvalid={hasError}
      p={3}
      rounded="xl"
      w={'full'}
      border="2px"
      borderColor={
        hasError
          ? 'red.500'
          : 'primary.700'
      }
    >
      <FormLabel color="neutral.600" fontSize="sm">{label}</FormLabel>
      <Flex align={'center'} gap={2}>
        <Icon as={icon} boxSize={7} color="primary.700" />
        <Field as={Input}
          name={name}
          placeholder={placeholder}
          fontSize="md"
          fontWeight="semibold"
          color="primary.800"
          px={2}
          {...defaultBehaviorInputProps}
        />
        <Text
          color="primary.800"
          fontWeight="bold"
          fontSize="sm"
        >
          {unit}
        </Text>
      </Flex>
      <FormErrorMessage fontSize={'xs'}>{errors[valueName]}</FormErrorMessage>
    </FormControl>
  );
};

export default function EMRFormVitalSign() {
  return (
    <Flex direction={'column'} gap={3}>
      <Text fontWeight={'bold'}>
        {vitalSignHeader.label}
      </Text>

      <Flex w="full" gap={'1rem'} wrap="wrap" align={'start'}>
        {vitalSignInput.map((input, index) => (
          <Flex
            key={index}
            w={{
              base: "100%",
              md: index < 3
                ? "calc(33.33% - 1rem)"
                : "calc(50% - 1rem)"
            }}
          >
            <InputFrame
              label={input.label}
              icon={input.icon}
              name={input.name}
              placeholder={input.placeholder}
              unit={input.unit}
            />
          </Flex>
        ))}
      </Flex>
    </Flex>
  )
}