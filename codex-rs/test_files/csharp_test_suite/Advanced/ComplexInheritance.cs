using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace TestApp.Advanced
{
    // Multiple interface implementations
    public interface IReadable
    {
        Task<string> ReadAsync();
    }

    public interface IWritable
    {
        Task WriteAsync(string content);
    }

    public interface ISeekable
    {
        void Seek(long position);
        long Position { get; }
    }

    public interface IDisposableResource : IDisposable
    {
        bool IsDisposed { get; }
    }

    // Complex inheritance with multiple interfaces
    public abstract class BaseStream : IReadable, IWritable, ISeekable, IDisposableResource
    {
        protected long _position;
        protected bool _disposed;

        public virtual long Position => _position;
        public bool IsDisposed => _disposed;

        public abstract Task<string> ReadAsync();
        public abstract Task WriteAsync(string content);

        public virtual void Seek(long position)
        {
            if (_disposed) throw new ObjectDisposedException(nameof(BaseStream));
            _position = position;
        }

        protected virtual void Dispose(bool disposing)
        {
            if (!_disposed)
            {
                if (disposing)
                {
                    // Dispose managed resources
                }
                _disposed = true;
            }
        }

        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }
    }

    // Concrete implementation with method overrides
    public class FileStream : BaseStream
    {
        private readonly string _fileName;

        public FileStream(string fileName)
        {
            _fileName = fileName ?? throw new ArgumentNullException(nameof(fileName));
        }

        public override async Task<string> ReadAsync()
        {
            if (_disposed) throw new ObjectDisposedException(nameof(FileStream));
            
            await Task.Delay(10); // Simulate file I/O
            return $"Content from {_fileName} at position {_position}";
        }

        public override async Task WriteAsync(string content)
        {
            if (_disposed) throw new ObjectDisposedException(nameof(FileStream));
            if (content == null) throw new ArgumentNullException(nameof(content));

            await Task.Delay(10); // Simulate file I/O
            _position += content.Length;
        }

        public override void Seek(long position)
        {
            if (position < 0) throw new ArgumentOutOfRangeException(nameof(position));
            base.Seek(position);
        }

        protected override void Dispose(bool disposing)
        {
            if (disposing)
            {
                // Close file handles, etc.
                Console.WriteLine($"Closing file: {_fileName}");
            }
            base.Dispose(disposing);
        }
    }

    // Generic abstract classes with constraints
    public abstract class Repository<TEntity, TKey> : IDisposable
        where TEntity : class, IEntity<TKey>
        where TKey : IEquatable<TKey>
    {
        protected readonly List<TEntity> _entities = new();
        private bool _disposed;

        public abstract Task<TEntity?> GetByIdAsync(TKey id);
        public abstract Task<IEnumerable<TEntity>> GetAllAsync();
        public abstract Task AddAsync(TEntity entity);
        public abstract Task UpdateAsync(TEntity entity);
        public abstract Task DeleteAsync(TKey id);

        protected virtual void ValidateEntity(TEntity entity)
        {
            if (entity == null) throw new ArgumentNullException(nameof(entity));
        }

        protected virtual void ThrowIfDisposed()
        {
            if (_disposed) throw new ObjectDisposedException(GetType().Name);
        }

        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        protected virtual void Dispose(bool disposing)
        {
            if (!_disposed)
            {
                if (disposing)
                {
                    _entities.Clear();
                }
                _disposed = true;
            }
        }
    }

    // Concrete generic repository implementation
    public class InMemoryRepository<TEntity, TKey> : Repository<TEntity, TKey>
        where TEntity : class, IEntity<TKey>
        where TKey : IEquatable<TKey>
    {
        public override async Task<TEntity?> GetByIdAsync(TKey id)
        {
            ThrowIfDisposed();
            await Task.Delay(1); // Simulate async operation
            
            foreach (var entity in _entities)
            {
                if (entity.Id.Equals(id))
                    return entity;
            }
            return null;
        }

        public override async Task<IEnumerable<TEntity>> GetAllAsync()
        {
            ThrowIfDisposed();
            await Task.Delay(1);
            return new List<TEntity>(_entities);
        }

        public override async Task AddAsync(TEntity entity)
        {
            ThrowIfDisposed();
            ValidateEntity(entity);
            
            await Task.Delay(1);
            _entities.Add(entity);
        }

        public override async Task UpdateAsync(TEntity entity)
        {
            ThrowIfDisposed();
            ValidateEntity(entity);
            
            var existing = await GetByIdAsync(entity.Id);
            if (existing == null)
                throw new InvalidOperationException($"Entity with ID {entity.Id} not found");

            var index = _entities.IndexOf(existing);
            _entities[index] = entity;
        }

        public override async Task DeleteAsync(TKey id)
        {
            ThrowIfDisposed();
            var entity = await GetByIdAsync(id);
            if (entity != null)
            {
                _entities.Remove(entity);
            }
        }
    }

    // Complex inheritance hierarchy with virtual and abstract methods
    public abstract class Vehicle
    {
        public string Make { get; protected set; }
        public string Model { get; protected set; }
        public int Year { get; protected set; }

        protected Vehicle(string make, string model, int year)
        {
            Make = make ?? throw new ArgumentNullException(nameof(make));
            Model = model ?? throw new ArgumentNullException(nameof(model));
            Year = year;
        }

        public abstract void Start();
        public abstract void Stop();
        public virtual void Honk() => Console.WriteLine("Beep beep!");
        
        public virtual string GetInfo() => $"{Year} {Make} {Model}";
    }

    public abstract class MotorVehicle : Vehicle
    {
        public string EngineType { get; protected set; }
        public double FuelCapacity { get; protected set; }

        protected MotorVehicle(string make, string model, int year, string engineType, double fuelCapacity)
            : base(make, model, year)
        {
            EngineType = engineType ?? throw new ArgumentNullException(nameof(engineType));
            FuelCapacity = fuelCapacity;
        }

        public abstract void Refuel(double amount);
        public virtual double GetFuelLevel() => FuelCapacity * 0.5; // Default implementation

        public override string GetInfo() => $"{base.GetInfo()} - Engine: {EngineType}";
    }

    public class Car : MotorVehicle
    {
        public int NumberOfDoors { get; }
        private double _currentFuel;

        public Car(string make, string model, int year, string engineType, double fuelCapacity, int numberOfDoors)
            : base(make, model, year, engineType, fuelCapacity)
        {
            NumberOfDoors = numberOfDoors;
            _currentFuel = fuelCapacity * 0.5;
        }

        public override void Start()
        {
            if (_currentFuel <= 0)
                throw new InvalidOperationException("Cannot start car: no fuel");
            Console.WriteLine($"Starting {Make} {Model}");
        }

        public override void Stop()
        {
            Console.WriteLine($"Stopping {Make} {Model}");
        }

        public override void Refuel(double amount)
        {
            if (amount < 0) throw new ArgumentException("Fuel amount cannot be negative");
            _currentFuel = Math.Min(_currentFuel + amount, FuelCapacity);
        }

        public override double GetFuelLevel() => _currentFuel;

        public override void Honk() => Console.WriteLine("Car horn: HONK HONK!");

        public override string GetInfo() => $"{base.GetInfo()} - Doors: {NumberOfDoors}";
    }

    public class Truck : MotorVehicle
    {
        public double CargoCapacity { get; }
        private double _currentFuel;
        private double _currentCargo;

        public Truck(string make, string model, int year, string engineType, double fuelCapacity, double cargoCapacity)
            : base(make, model, year, engineType, fuelCapacity)
        {
            CargoCapacity = cargoCapacity;
            _currentFuel = fuelCapacity * 0.3;
        }

        public override void Start()
        {
            if (_currentFuel <= 0)
                throw new InvalidOperationException("Cannot start truck: no fuel");
            Console.WriteLine($"Starting truck {Make} {Model}");
        }

        public override void Stop()
        {
            Console.WriteLine($"Stopping truck {Make} {Model}");
        }

        public override void Refuel(double amount)
        {
            if (amount < 0) throw new ArgumentException("Fuel amount cannot be negative");
            _currentFuel = Math.Min(_currentFuel + amount, FuelCapacity);
        }

        public override double GetFuelLevel() => _currentFuel;

        public override void Honk() => Console.WriteLine("Truck horn: HOOOOOONK!");

        public void LoadCargo(double weight)
        {
            if (weight < 0) throw new ArgumentException("Cargo weight cannot be negative");
            _currentCargo = Math.Min(_currentCargo + weight, CargoCapacity);
        }

        public void UnloadCargo(double weight)
        {
            if (weight < 0) throw new ArgumentException("Cargo weight cannot be negative");
            _currentCargo = Math.Max(_currentCargo - weight, 0);
        }

        public double GetCargoWeight() => _currentCargo;

        public override string GetInfo() => $"{base.GetInfo()} - Cargo: {_currentCargo}/{CargoCapacity}";
    }

    // Covariance and contravariance examples
    public interface IProducer<out T>
    {
        T Produce();
    }

    public interface IConsumer<in T>
    {
        void Consume(T item);
    }

    public interface IProcessor<in TInput, out TOutput>
    {
        TOutput Process(TInput input);
    }

    public class StringProducer : IProducer<string>
    {
        public string Produce() => "Hello World";
    }

    public class ObjectConsumer : IConsumer<object>
    {
        public void Consume(object item) => Console.WriteLine(item?.ToString() ?? "null");
    }

    public class StringToIntProcessor : IProcessor<string, int>
    {
        public int Process(string input) => input?.Length ?? 0;
    }

    // Demonstration of covariance/contravariance usage
    public class VarianceExamples
    {
        public void DemonstrateCovariance()
        {
            IProducer<string> stringProducer = new StringProducer();
            IProducer<object> objectProducer = stringProducer; // Covariance
            object result = objectProducer.Produce();
        }

        public void DemonstrateContravariance()
        {
            IConsumer<object> objectConsumer = new ObjectConsumer();
            IConsumer<string> stringConsumer = objectConsumer; // Contravariance
            stringConsumer.Consume("Hello");
        }

        public void DemonstrateProcessorVariance()
        {
            IProcessor<string, int> processor = new StringToIntProcessor();
            IProcessor<object, object> generalProcessor = processor; // Both variance types
            object result = generalProcessor.Process("test");
        }
    }
}