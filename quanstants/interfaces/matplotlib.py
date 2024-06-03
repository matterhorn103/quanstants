 
from matplotlib import pyplot
from matplotlib.units import ConversionInterface, AxisInfo, registry as mpl_registry
#from matplotlib.pyplot import np  # if we need numpy

from ..abstract_quantity import AbstractQuantity
#from ..array import QuanstantsArray


class QuanstantsMatplotlibInterface(ConversionInterface):
    """Interface to matplotlib."""

    # The API is not very extensively documented or maintained
    # but it is possible to plot things like this

    # Frustratingly, mpl behaves differently when quanstants objects are passed to
    # `plt.plot(x, y)` or `ax.plot(x, y)` vs to `plt.scatter(x, y)` or `ax.scatter(x, y)`
    # With `plot`, x and y become ndarray[x] and ndarray[y],
    # whereas with `scatter`, x and y stay as their type,
    # and these are the objects that are passed as obj in the below

    @staticmethod
    #def convert(obj: AbstractQuantity | QuanstantsArray, unit, axis):
    def convert(obj: AbstractQuantity, unit, axis):
        """Convert a quanstants object (or an ndarray of) to an array of numbers."""
        if isinstance(obj, AbstractQuantity):
            return obj.to(unit).number
        #elif isinstance(obj, QuanstantsArray):
            #return obj.to(unit).numbers
        else:
            # Assume ndarray of quantities
            if isinstance(obj[0], AbstractQuantity):
                return [q.to(unit).number for q in obj]
            #elif isinstance(obj[0], QuanstantsArray):
                #return obj[0].to(unit).numbers
            else:
                raise TypeError

    @staticmethod
    def axisinfo(unit, axis):
        """Provide information for axis formatting including a label with the unit."""
        current_label = axis.get_label_text()

        return AxisInfo(label=f"{current_label} ({unit.symbol})")

    @staticmethod
    #def default_units(obj: AbstractQuantity | QuanstantsArray, axis):
    def default_units(obj: AbstractQuantity, axis):
        """mpl docs: 'Return the default unit for x or None for the given axis.'"""
        # Not sure what this is meant to do really
        # It seems it's just supposed to return the unit of object x so that mpl knows
        # how to extract a unit from an object passed to it
        #if isinstance(obj, (AbstractQuantity, QuanstantsArray)):
        if isinstance(obj, AbstractQuantity):
            return obj.unit
        else:
            return obj[0].unit


# Register our custom interface as the interface for quanstants quantities and arrays
mpl_registry[AbstractQuantity] = QuanstantsMatplotlibInterface()
#mpl_registry[QuanstantsArray] = QuanstantsMatplotlibInterface()
